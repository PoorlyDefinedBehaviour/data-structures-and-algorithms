#pragma once

#include <vector>
#include <memory>

struct Point
{
  size_t x;
  size_t y;
};

struct Region
{
  Point position;
  unsigned int width;
  unsigned int height;

  Region(size_t x, size_t y, unsigned int w, unsigned int h) : position({x, y}), width(w), height(h) {}

  auto contains(Point const &point) const -> bool
  {
    return point.x >= position.x - width &&
           point.x <= position.x + width &&
           point.y >= position.y - height &&
           point.y <= position.y + height;
  }

  auto intersects(Region const &boundary) const -> bool
  {
    return !(boundary.position.x - boundary.width > position.x + width ||
             boundary.position.x + boundary.width < position.x - width ||
             boundary.position.y - boundary.height > position.y + height ||
             boundary.position.y + boundary.height < position.y - height);
  }
};

class QuadTree
{
private:
  unsigned int const max_capacity = 4;

  Region boundary;

  std::vector<Point> points;

  std::unique_ptr<QuadTree> northwest = nullptr;
  std::unique_ptr<QuadTree> northeast = nullptr;
  std::unique_ptr<QuadTree> southwest = nullptr;
  std::unique_ptr<QuadTree> southeast = nullptr;

  auto impl_query(Region const &boundary, std::vector<Point> &result) const -> std::vector<Point>
  {
    if (!this->boundary.intersects(boundary))
    {
      return result;
    }

    for (auto const &point : points)
    {
      if (boundary.contains(point))
      {
        result.emplace_back(point);
      }
    }

    if (is_sub_divided())
    {
      northwest->impl_query(boundary, result);
      northeast->impl_query(boundary, result);
      southwest->impl_query(boundary, result);
      southeast->impl_query(boundary, result);
    }

    return result;
  }

public:
  QuadTree(Region const &region) : boundary(region) {}

  auto query(Region &&boundary) -> std::vector<Point>
  {
    std::vector<Point> result;
    result.reserve(4);
    return impl_query(std::forward<Region>(boundary), result);
  }

  auto is_sub_divided() const -> bool
  {
    return northwest != nullptr;
  }

  auto sub_divide() -> void
  {
    northeast = std::make_unique<QuadTree>(Region(boundary.position.x + boundary.width / 2,
                                                  boundary.position.y - boundary.height / 2,
                                                  boundary.width / 2,
                                                  boundary.height / 2));

    northwest = std::make_unique<QuadTree>(Region(boundary.position.x - boundary.width / 2,
                                                  boundary.position.y - boundary.height / 2,
                                                  boundary.width / 2,
                                                  boundary.height / 2));

    southeast = std::make_unique<QuadTree>(Region(boundary.position.x + boundary.width / 2,
                                                  boundary.position.y + boundary.height / 2,
                                                  boundary.width / 2,
                                                  boundary.height / 2));

    southwest = std::make_unique<QuadTree>(Region(boundary.position.x - boundary.width / 2,
                                                  boundary.position.y + boundary.height / 2,
                                                  boundary.width / 2,
                                                  boundary.height / 2));
  }

  auto insert(Point const &point) -> bool
  {
    if (!boundary.contains(point))
    {
      return false;
    }

    if (points.size() < max_capacity)
    {
      points.emplace_back(point);
      return true;
    }

    if (!is_sub_divided())
    {
      sub_divide();
    }

    return northeast->insert(point) ||
           northwest->insert(point) ||
           southeast->insert(point) ||
           southwest->insert(point);
  }
};

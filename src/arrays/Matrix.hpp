#pragma once

#include <stdexcept>
#include <vector>
#include <iostream>

template <typename T>
class Matrix
{
private:
  void assert_same_size(Matrix const &one, Matrix const &other)
  {
    if (one.rows != other.rows || one.columns != other.columns)
      throw std::length_error("Matrix A size must match Matrix B size");
  }

  void assert_dot_product_is_possible(Matrix const &one, Matrix const &other)
  {
    if (one.columns != other.rows)
      throw std::length_error("Matrix A columns must match Matrix B rows");
  }

  void assert_index_is_valid(size_t index)
  {
    if (index < 0 || index > this->rows - 1)
      throw std::out_of_range("Invalid index");
  }

  size_t rows;
  size_t columns;
  std::vector<std::vector<T>> elements;

public:
  Matrix() = default;

  Matrix(size_t _rows, size_t _columns)
      : rows(_rows), columns(_columns)
  {
    this->resize(this->rows, this->columns);
  }

  Matrix<T> &resize(int rows, int columns)
  {
    this->elements = std::vector<std::vector<T>>(rows, std::vector<T>(columns));

    this->rows = rows;
    this->columns = columns;

    return *this;
  }

  size_t get_rows() const
  {
    return this->rows;
  }

  size_t get_columns() const
  {
    return this->columns;
  }

  Matrix<T> &operator=(std::vector<std::vector<T>> const elements)
  {
    this->elements = elements;
  }

  std::vector<T> &operator[](size_t index)
  {
    assert_index_is_valid(index);

    return this->elements[index];
  }

  const std::vector<T> &operator[](size_t index) const
  {
    assert_index_is_valid(index);

    return this->elements[index];
  }

  Matrix operator*(Matrix const &other)
  {
    assert_dot_product_is_possible(*this, other);

    Matrix<T> result(rows, other.columns);

    for (size_t i = 0; i < rows; ++i)
    {
      for (size_t j = 0; j < other.columns; ++j)
      {

        auto sum = 0.0f;
        for (size_t k = 0; k < columns; ++k)
        {
          sum += elements[i][k] * other.elements[k][j];
        }
        result[i][j] = sum;
      }
    }

    return result;
  }

  Matrix<T> operator*(Vector3D const &other)
  {
    Matrix<T> new_matrix(3, 1);
    new_matrix[0][0] = other.x;
    new_matrix[1][0] = other.y;
    new_matrix[2][0] = other.z;

    return *this * new_matrix;
  }

  void operator*=(T value)
  {
    *this = *this * value;
  }

  Matrix<T> operator*(T value)
  {
    return map([&value](auto const &element) -> T { return element * value; });
  }

  void operator/=(T value)
  {
    *this = *this / value;
  }

  Matrix<T> operator/(T value)
  {
    return map([&value](auto const &element) -> T { return element / value; });
  }

  void operator+=(T value)
  {
    *this = *this + value;
  }

  Matrix<T> operator+(T value)
  {
    return map([&value](auto const &element) -> T { return element + value; });
  }

  void operator-=(T value)
  {
    *this = *this - value;
  }

  Matrix<T> operator-(T value)
  {
    return map([&value](auto const &element) -> T { return element - value; });
  }

  Matrix<T> element_wise_add(Matrix const &other)
  {
    assert_same_size(*this, other);

    Matrix<T> result(this->rows, this->columns);

    for (size_t i = 0; i < this->rows; ++i)
      for (size_t j = 0; j < this->columns; ++j)
        result[i][j] = this->elements[i][j] + other[i][j];

    return result;
  }

  Matrix<T> element_wise_subtract(Matrix const &other)
  {
    assert_same_size(*this, other);

    Matrix<T> result(this->rows, this->columns);

    for (size_t i = 0; i < this->rows; ++i)
      for (size_t j = 0; j < this->columns; ++j)
        result[i][j] = this->elements[i][j] - other[i][j];

    return result;
  }

  Matrix<T> element_wise_multiply(Matrix const &other)
  {
    assert_same_size(*this, other);

    Matrix<T> result(this->rows, this->columns);

    for (size_t i = 0; i < this->rows; ++i)
      for (size_t j = 0; j < this->columns; ++j)
        result[i][i] = this->elements[i][j] * other[i][j];

    return result;
  }

  Matrix<T> element_wise_divide(Matrix const &other)
  {
    assert_same_size(*this, other);

    Matrix<T> result(this->rows, this->columns);

    for (size_t i = 0; i < this->rows; ++i)
      for (size_t j = 0; j < this->columns; ++j)
        result[i][i] = this->elements[i][j] / other[i][j];

    return result;
  }

  Matrix<T> transpose()
  {
    Matrix<T> result(this->columns, this->rows);

    for (size_t i = 0; i < result.rows; i++)
    {
      for (size_t j = 0; j < result.columns; j++)
      {
        result[j][i] = elements[i][j];
      }
    }
    return result;
  }

  std::vector<T> to_stdvector()
  {
    std::vector<T> result;
    result.reserve(this->rows * this->columns);

    for (size_t i = 0; i < this->rows; ++i)
      for (size_t j = 0; j < this->columns; ++j)
        result.emplace_back(this->elements[i][j]);

    return result;
  }

  template <typename lambda>
  Matrix<T> map(lambda const &func)
  {
    Matrix<T> result(this->rows, this->columns);

    for (size_t i = 0; i < result.rows; i++)
    {
      for (size_t j = 0; j < result.columns; j++)
      {
        result[i][j] = func(this->elements[i][j]);
      }
    }
    return result;
  }

  template <typename lambda>
  Matrix<T> &fill(lambda const &func)
  {
    for (size_t i = 0; i < this->rows; ++i)
      for (size_t j = 0; j < this->columns; ++j)
        this->elements[i][j] = func();

    return *this;
  }

  void print()
  {
    std::cout << this->rows << " x " << this->columns << '\n';
    for (size_t i = 0; i < this->rows; ++i)
    {
      for (size_t j = 0; j < this->columns; ++j)
      {
        std::cout << this->elements[i][j] << ' ';
      }
      std::cout << '\n';
    }
    std::cout << '\n';
  }
};
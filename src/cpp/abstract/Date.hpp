#pragma once

#include <iostream>
#include <string>
#include <map>
#include <cassert>
#include <chrono>

class Date
{
  long day;
  long month;
  long year;

  struct MonthInfo
  {
    std::string name;
    long days;
  };

  std::map<long, MonthInfo> months =
      {{1, {"January", 31}},
       {2, {"February", 28}},
       {3, {"March", 31}},
       {4, {"April", 30}},
       {5, {"May", 31}},
       {6, {"June", 30}},
       {7, {"July", 31}},
       {8, {"August", 31}},
       {9, {"September", 30}},
       {10, {"October", 31}},
       {11, {"November", 30}},
       {12, {"December", 31}}};

  constexpr auto is_leap(long year) const -> bool { return year % 4 == 0; }

  auto increment() -> void
  {
    if (++day > months[month].days + (month == 2 && is_leap(year)))
    {
      day = 1;
      if (++month > 12)
      {
        ++year;
        month = 1;
      }
    }
  }

  auto decrement() -> void
  {
    if (--day < 1)
    {
      day = months[month].days;
      if (--month < 1)
      {
        --year;
        month = 12;
        day = months[12].days;
      }
    }
  }

  friend auto operator<<(std::ostream &stream, Date const &date) -> std::ostream &
  {
    stream << date.day << '/' << date.month << '/' << date.year << '\n';
    return stream;
  }

public:
  explicit Date(long day, long month, long year) : day(day), month(month), year(year)
  {
    assert(day > -1 && month > -1 && year > -1);
  }

  static auto now() -> std::chrono::time_point<std::chrono::high_resolution_clock>
  {
    return std::chrono::high_resolution_clock::now();
  }

  static auto to_microseconds(std::chrono::time_point<std::chrono::high_resolution_clock> const &clock) -> long long
  {
    return std::chrono::time_point_cast<std::chrono::microseconds>(clock).time_since_epoch().count();
  }

  static auto to_milliseconds(std::chrono::time_point<std::chrono::high_resolution_clock> const &clock) -> long long
  {
    return std::chrono::time_point_cast<std::chrono::milliseconds>(clock).time_since_epoch().count();
  }

  static auto to_seconds(std::chrono::time_point<std::chrono::high_resolution_clock> const &clock) -> long long
  {
    return std::chrono::time_point_cast<std::chrono::seconds>(clock).time_since_epoch().count();
  }

  static auto to_minutes(std::chrono::time_point<std::chrono::high_resolution_clock> const &clock) -> long long
  {
    return std::chrono::time_point_cast<std::chrono::minutes>(clock).time_since_epoch().count();
  }

  static auto to_hours(std::chrono::time_point<std::chrono::high_resolution_clock> const &clock) -> long long
  {
    return std::chrono::time_point_cast<std::chrono::hours>(clock).time_since_epoch().count();
  }

  static auto to_days(Date date) -> long
  {
    long days = 0;

    while (date.year != 0)
    {
      ++days;
      date.decrement();
    }

    return days;
  }

  auto operator++() -> Date &
  {
    increment();

    return *this;
  }

  auto operator++(int) -> Date
  {
    auto temp = *this;
    increment();
    return temp;
  }

  auto operator--() -> Date &
  {
    decrement();

    return *this;
  }

  auto operator--(int) -> Date
  {
    auto temp = *this;
    decrement();
    return temp;
  }

  auto operator-(Date const &other) -> Date
  {
    return *this - to_days(other);
  }

  auto operator+(Date const &other) -> Date
  {
    return *this + to_days(other);
  }

  auto operator+(long days) -> Date
  {
    for (long i = 0; i < days; ++i)
    {
      increment();
    }

    return *this;
  }

  auto operator-(long days) -> Date
  {
    for (long i = 0; i < days; ++i)
    {
      decrement();
    }

    return *this;
  }
};

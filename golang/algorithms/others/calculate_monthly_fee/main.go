package main

import (
	"fmt"
	"time"
)

type User struct {
	ActivatedOn   time.Time
	DeactivatedOn time.Time
}

type Plan struct {
	MonthlyFeePerUser int
}

func wasUserActiveOnDate(date time.Time, user *User) bool {
	if date.Equal(user.ActivatedOn) {
		return true
	}

	if date.Before(user.ActivatedOn) {
		if user.DeactivatedOn.IsZero() {
			return true
		}

		if user.DeactivatedOn.Equal(date) {
			return true
		}

		if user.DeactivatedOn.After(date) {
			return true
		}
	}

	return false
}

func numberOfDaysUserWasActive(date time.Time, user *User) int {
	if user.ActivatedOn.IsZero() {
		return 0
	}

	if user.DeactivatedOn.IsZero() {
		if user.ActivatedOn.Equal(date) {
			_, month, year := date.Date()
			return daysIn(month, year)
		}

		return int(date.Sub(user.ActivatedOn).Hours() / 24)
	}

	return int(user.DeactivatedOn.Sub(user.ActivatedOn).Hours() / 24)
}

func daysIn(m time.Month, year int) int {
	return time.Date(year, m+1, 0, 0, 0, 0, 0, time.UTC).Day()
}

func Calculate(monthYear string, plan *Plan, users *[]User) float64 {
	if plan == nil || len(*users) == 0 {
		return 0.0
	}

	date, _ := time.Parse("2006-01", monthYear)
	_, month, year := date.Date()
	daysInAMonth := daysIn(month, year)
	dailyFee := float64(plan.MonthlyFeePerUser) / float64(daysInAMonth)

	total := 0.0
	for _, user := range *users {
		total += float64(numberOfDaysUserWasActive(date, &user)) * dailyFee
	}

	return total
}

func main() {
	fmt.Println("hello world")
}

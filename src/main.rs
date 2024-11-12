package main

import (
    "fmt"
    "net/http"
    "sort"
    "time"
)

// Функция для вычисления медианы
func median(arr []int64) int64 {
    sort.Slice(arr, func(i, j int) bool {
        return arr[i] < arr[j]
    })
    length := len(arr)
    if length%2 == 0 {
        return (arr[length/2-1] + arr[length/2]) / 2
    }
    return arr[length/2]
}

func main() {
    repeats := 10 // Количество повторов
    var times []int64

    // Первый запрос для прогрева
    fmt.Println("Skipping first request (warm-up)...")
    http.Get("https://api.bybit.com/v2/public/time")

    // Основные запросы
    for i := 0; i < repeats; i++ {
        start := time.Now()

        resp, err := http.Get("https://api.bybit.com/v2/public/time")
        if err != nil {
            fmt.Printf("Request %d failed: %v\n", i+1, err)
            continue
        }
        resp.Body.Close()

        duration := time.Since(start).Milliseconds()
        times = append(times, duration)

        fmt.Printf("Request %d time: %d ms\n", i+1, duration)
    }

    // Вычисление минимального, максимального и медианного времени
    minTime := times[0]
    maxTime := times[0]
    for _, t := range times {
        if t < minTime {
            minTime = t
        }
        if t > maxTime {
            maxTime = t
        }
    }
    medianTime := median(times)

    fmt.Printf("\nResults over %d requests (excluding warm-up):\n", repeats)
    fmt.Printf("Min time: %d ms\n", minTime)
    fmt.Printf("Max time: %d ms\n", maxTime)
    fmt.Printf("Median time: %d ms\n", medianTime)
}

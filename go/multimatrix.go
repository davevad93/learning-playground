package main

import (
	"fmt"
	"math/rand"
	"runtime"
	"sync"
	"time"
)

// Function to generate a random matrix of size rows x cols
func generateMatrix(rows, cols int) [][]int {
	matrix := make([][]int, rows)
	for i := range matrix {
		matrix[i] = make([]int, cols)
		for j := range matrix[i] {
			matrix[i][j] = rand.Intn(100)
		}
	}
	return matrix
}

// Function to multiply two matrices concurrently
func multiplyMatrices(a, b [][]int, result [][]int, numWorkers int) {
	rows := len(a)
	cols := len(b[0])

	var wg sync.WaitGroup
	tasks := make(chan [2]int, rows*cols)

	// Worker function to compute a single element in the result matrix
	worker := func() {
		defer wg.Done()
		for task := range tasks {
			i, j := task[0], task[1]
			sum := 0
			for k := 0; k < len(b); k++ {
				sum += a[i][k] * b[k][j]
			}
			result[i][j] = sum
		}
	}

	// Start workers
	for i := 0; i < numWorkers; i++ {
		wg.Add(1)
		go worker()
	}

	// Send tasks to the workers
	for i := 0; i < rows; i++ {
		for j := 0; j < cols; j++ {
			tasks <- [2]int{i, j}
		}
	}
	close(tasks)

	// Wait for all workers to finish
	wg.Wait()
}

func main() {
	runtime.GOMAXPROCS(runtime.NumCPU()) 

	rand.Seed(time.Now().UnixNano())

	// Define matrix sizes
	const size = 500
	a := generateMatrix(size, size)
	b := generateMatrix(size, size)
	result := make([][]int, size)
	for i := range result {
		result[i] = make([]int, size)
	}

	start := time.Now()

	// Multiply matrices using a number of workers equal to the number of CPU cores
	numWorkers := runtime.NumCPU()
	multiplyMatrices(a, b, result, numWorkers)

	elapsed := time.Since(start)
	fmt.Printf("Matrix multiplication completed in %s using %d workers.\n", elapsed, numWorkers)
}

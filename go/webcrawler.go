package main

import (
    "fmt"
    "io/ioutil"
    "net/http"
    "runtime"
    "sync"
    "time"
)

// Worker function that performs the task
func worker(id int, jobs <-chan string, results chan<- string, wg *sync.WaitGroup) {
    defer wg.Done()
    for url := range jobs {
        fmt.Printf("Worker %d starting to fetch URL: %s\n", id, url)
        body, err := fetchURL(url)
        if err != nil {
            results <- fmt.Sprintf("Worker %d failed to fetch %s: %v", id, url, err)
            continue
        }
        results <- fmt.Sprintf("Worker %d finished fetching %s, length: %d bytes", id, url, len(body))
        fmt.Printf("Worker %d finished URL: %s\n", id, url)
    }
}

// Fetch a URL and return its body as a string
func fetchURL(url string) (string, error) {
    resp, err := http.Get(url)
    if err != nil {
        return "", err
    }
    defer resp.Body.Close()

    body, err := ioutil.ReadAll(resp.Body)
    if err != nil {
        return "", err
    }

    return string(body), nil
}

func main() {
    runtime.GOMAXPROCS(runtime.NumCPU())

    urls := []string{
        "http://example.com",
    }

    numWorkers := len(urls)
    jobs := make(chan string, len(urls))
    results := make(chan string, len(urls))

    var wg sync.WaitGroup

    // Start the workers
    for i := 1; i <= numWorkers; i++ {
        wg.Add(1)
        go worker(i, jobs, results, &wg)
    }

    // Send jobs to the workers
    for _, url := range urls {
        jobs <- url
    }
    close(jobs)

    // Wait for all workers to finish
    wg.Wait()
    close(results)

    // Collect results
    for result := range results {
        fmt.Println(result)
    }
}

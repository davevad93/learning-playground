#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>
#include <complex.h>

// Define constants for the Mandelbrot set
#define WIDTH 800
#define HEIGHT 800
#define MAX_ITER 1000
#define NUM_THREADS 8

typedef struct {
    int start_row;
    int end_row;
    int width;
    int height;
    int max_iter;
    int *output;
} MandelbrotParams;

void *compute_mandelbrot(void *params) {
    MandelbrotParams *p = (MandelbrotParams *)params;
    for (int y = p->start_row; y < p->end_row; y++) {
        for (int x = 0; x < p->width; x++) {
            double complex c = (x - p->width / 2.0) * 4.0 / p->width + 
                               (y - p->height / 2.0) * 4.0 / p->height * I;
            double complex z = 0;
            int iter;
            for (iter = 0; iter < p->max_iter && cabs(z) < 2.0; iter++) {
                z = z * z + c;
            }
            p->output[y * p->width + x] = iter;
        }
    }
    return NULL;
}

int main() {
    // Allocate memory for the output image
    int *output = (int *)malloc(WIDTH * HEIGHT * sizeof(int));
    if (output == NULL) {
        fprintf(stderr, "Error: Could not allocate memory.\n");
        return 1;
    }

    // Create threads
    pthread_t threads[NUM_THREADS];
    MandelbrotParams params[NUM_THREADS];
    int rows_per_thread = HEIGHT / NUM_THREADS;

    for (int i = 0; i < NUM_THREADS; i++) {
        params[i].start_row = i * rows_per_thread;
        params[i].end_row = (i + 1) * rows_per_thread;
        params[i].width = WIDTH;
        params[i].height = HEIGHT;
        params[i].max_iter = MAX_ITER;
        params[i].output = output;

        if (pthread_create(&threads[i], NULL, compute_mandelbrot, &params[i]) != 0) {
            fprintf(stderr, "Error: Could not create thread.\n");
            free(output);
            return 1;
        }
    }

    // Wait for all threads to finish
    for (int i = 0; i < NUM_THREADS; i++) {
        pthread_join(threads[i], NULL);
    }

    // Output the results as a PGM image
    FILE *fp = fopen("mandelbrot.pgm", "wb");
    if (fp == NULL) {
        fprintf(stderr, "Error: Could not open file for writing.\n");
        free(output);
        return 1;
    }
    fprintf(fp, "P2\n%d %d\n%d\n", WIDTH, HEIGHT, MAX_ITER);
    for (int i = 0; i < HEIGHT; i++) {
        for (int j = 0; j < WIDTH; j++) {
            fprintf(fp, "%d ", output[i * WIDTH + j]);
        }
        fprintf(fp, "\n");
    }
    fclose(fp);

    // Free the allocated memory
    free(output);

    return 0;
}

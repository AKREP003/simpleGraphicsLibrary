
#include "cuda_runtime.h"
#include "device_launch_parameters.h"
#include <math_functions.h>
#include <stdio.h>

#define DLLEXPORT __declspec(dllexport)




extern "C" cudaError_t DLLEXPORT addWithCuda(int* c, const int* a, const int* b, unsigned int size);


__global__ void addKernel(int *c, const int *a, const int *b)
{
    int i = threadIdx.x;
    c[i] = a[i] + b[i];
}

typedef struct InfLine {

    float slope;
    int constant;

} InfLine;



typedef struct graphicTriangle {

    InfLine lines[2];
    int coords[2];

} graphicTriangle;

__device__ int crossing_point(int x, InfLine line) {

    return (x * line.slope) + line.constant;

}

__global__ void renderTriangle(int* can, int *data, int WIDTH, int HEIGHT)
{
    
    graphicTriangle* triangles = reinterpret_cast<graphicTriangle*>(data);

    int8_t* canvas = reinterpret_cast<int8_t*>(can);

    int i = threadIdx.x;

    graphicTriangle triangle = triangles[i];

    int length = triangle.coords[1] - triangle.coords[0];

    for (int x = 0; x < length; x++) {
    
        int r_x = x + triangle.coords[0];

        if (r_x < 2 || r_x > WIDTH) { continue; }

        int y1 = crossing_point(x, triangle.lines[0]);
        int y2 = crossing_point(x, triangle.lines[1]);

        for (int y = min(y1, y2); y < max(y1, y2); y++) {

            if (y > 1 && y < HEIGHT - 2) {

                canvas[(((WIDTH * y) + r_x) * 4) + 0] = 127;
                canvas[(((WIDTH * y) + r_x) * 4) + 1] = 127;
                canvas[(((WIDTH * y) + r_x) * 4) + 2] = 127;
                canvas[(((WIDTH * y) + r_x) * 4) + 3] = 127;

            }

        }
    
    }

}




extern "C" cudaError_t DLLEXPORT drawTriangles(int* canvas, int WIDTH, int HEIGHT, const graphicTriangle* data, size_t sizeTri)
{
    
    
    int* dev_canvas = 0;

    int* dev_tri = 0;

    cudaError_t cudaStatus;


    cudaStatus = cudaSetDevice(0);
    if (cudaStatus != cudaSuccess) {
        fprintf(stderr, "cudaSetDevice failed!  Do you have a CUDA-capable GPU installed?");
        goto Error;
    }

    int current_size = WIDTH * HEIGHT * sizeof(int8_t) * 4;
    
    


    cudaStatus = cudaMalloc((void**)&dev_canvas, current_size);
    if (cudaStatus != cudaSuccess) {
        
        fprintf(stderr, "cudaMalloc failed! canvas. Error: %s\n", cudaGetErrorString(cudaStatus));
        goto Error;
    }

    cudaStatus = cudaMalloc((void**)&dev_tri, sizeTri * sizeof(graphicTriangle));
    if (cudaStatus != cudaSuccess) {
        printf("b");
        fprintf(stderr, "cudaMalloc failed! triangles");
        goto Error;
    }

    cudaStatus = cudaMemcpy(dev_canvas, canvas, current_size, cudaMemcpyHostToDevice);
    if (cudaStatus != cudaSuccess) {
        fprintf(stderr, "cudaMalloc failed! canva. Error: %s\n", cudaGetErrorString(cudaStatus));
        goto Error;
    }

    cudaStatus = cudaMemcpy(dev_tri, data, sizeTri * sizeof(graphicTriangle), cudaMemcpyHostToDevice);
    if (cudaStatus != cudaSuccess) {
        fprintf(stderr, "cudaMemcpy failed! triangles");
        goto Error;
    }

    renderTriangle<<<1, sizeTri >>>(dev_canvas, dev_tri, WIDTH, HEIGHT);

    cudaStatus = cudaGetLastError();
    if (cudaStatus != cudaSuccess) {
        fprintf(stderr, "addKernel launch failed: %s\n", cudaGetErrorString(cudaStatus));
        goto Error;
    }

    // cudaDeviceSynchronize waits for the kernel to finish, and returns
    // any errors encountered during the launch.
    cudaStatus = cudaDeviceSynchronize();
    if (cudaStatus != cudaSuccess) {
        fprintf(stderr, "cudaDeviceSynchronize returned error code %d after launching addKernel!\n", cudaStatus);
        goto Error;
    }

    // Copy output vector from GPU buffer to host memory.
    cudaStatus = cudaMemcpy(canvas, dev_canvas, current_size, cudaMemcpyDeviceToHost);
    if (cudaStatus != cudaSuccess) {
        fprintf(stderr, "cudaMalloc failed! canv. Error: %s %d\n", cudaGetErrorString(cudaStatus), current_size);goto Error;
    }

    cudaFree(dev_canvas);
    cudaFree(dev_tri);

    return cudaSuccess;
Error:

    return cudaStatus;

}

int main()
{
    const int arraySize = 5;
    const int a[arraySize] = { 1, 2, 3, 4, 5 };
    const int b[arraySize] = { 10, 20, 30, 40, 50 };
    int c[arraySize] = { 0 };

    // Add vectors in parallel.
    cudaError_t cudaStatus = addWithCuda(c, a, b, arraySize);
    if (cudaStatus != cudaSuccess) { fprintf(stderr, "addWithCuda failed!");return 1; }

    printf("{1,2,3,4,5} + {10,20,30,40,50} = {%d,%d,%d,%d,%d}\n",
        c[0], c[1], c[2], c[3], c[4]);

    // cudaDeviceReset must be called before exiting in order for profiling and
    // tracing tools such as Nsight and Visual Profiler to show complete traces.
    cudaStatus = cudaDeviceReset();
    if (cudaStatus != cudaSuccess) {
        fprintf(stderr, "cudaDeviceReset failed!");
        return 1;
    }

    return 0;
}

// Helper function for using CUDA to add vectors in parallel.
extern "C" cudaError_t DLLEXPORT addWithCuda(int* c, const int* a, const int* b, unsigned int size)
{
    int *dev_a = 0;
    int *dev_b = 0;
    int *dev_c = 0;
    cudaError_t cudaStatus;

    
    cudaStatus = cudaSetDevice(0);
    if (cudaStatus != cudaSuccess) {
        fprintf(stderr, "cudaSetDevice failed!  Do you have a CUDA-capable GPU installed?");
        goto Error;
    }

    // Allocate GPU buffers for three vectors (two input, one output)    .
    cudaStatus = cudaMalloc((void**)&dev_c, size * sizeof(int));
    if (cudaStatus != cudaSuccess) {
        fprintf(stderr, "cudaMalloc failed!");
        goto Error;
    }

    cudaStatus = cudaMalloc((void**)&dev_a, size * sizeof(int));
    if (cudaStatus != cudaSuccess) {
        fprintf(stderr, "cudaMalloc failed!");
        goto Error;
    }

    cudaStatus = cudaMalloc((void**)&dev_b, size * sizeof(int));
    if (cudaStatus != cudaSuccess) {
        fprintf(stderr, "cudaMalloc failed!");
        goto Error;
    }

    // Copy input vectors from host memory to GPU buffers.
    cudaStatus = cudaMemcpy(dev_a, a, size * sizeof(int), cudaMemcpyHostToDevice);
    if (cudaStatus != cudaSuccess) {
        fprintf(stderr, "cudaMemcpy failed!");
        goto Error;
    }

    cudaStatus = cudaMemcpy(dev_b, b, size * sizeof(int), cudaMemcpyHostToDevice);
    if (cudaStatus != cudaSuccess) {
        fprintf(stderr, "cudaMemcpy failed!");
        goto Error;
    }

    // Launch a kernel on the GPU with one thread for each element.
    addKernel<<<1, size>>>(dev_c, dev_a, dev_b);

    // Check for any errors launching the kernel
    cudaStatus = cudaGetLastError();
    if (cudaStatus != cudaSuccess) {
        fprintf(stderr, "addKernel launch failed: %s\n", cudaGetErrorString(cudaStatus));
        goto Error;
    }
    
    // cudaDeviceSynchronize waits for the kernel to finish, and returns
    // any errors encountered during the launch.
    cudaStatus = cudaDeviceSynchronize();
    if (cudaStatus != cudaSuccess) {
        fprintf(stderr, "cudaDeviceSynchronize returned error code %d after launching addKernel!\n", cudaStatus);
        goto Error;
    }

    // Copy output vector from GPU buffer to host memory.
    cudaStatus = cudaMemcpy(c, dev_c, size * sizeof(int), cudaMemcpyDeviceToHost);
    if (cudaStatus != cudaSuccess) {
        fprintf(stderr, "cudaMemcpy failed!");
        goto Error;
    }

Error:
    cudaFree(dev_c);
    cudaFree(dev_a);
    cudaFree(dev_b);
    
    return cudaStatus;
}


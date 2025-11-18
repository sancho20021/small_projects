# A Verified Thread-Safe Array in Rust
This library is written in [Verus](https://github.com/verus-lang/verus) verification framework.
It provides simple array api (create, get, and set), and the array can be safely shared across threads.
The safety (from data races) is accomplished via splitting a ghost set of array permissions into disjoint subsets.
Each permission is associated with each array cell. All permissions manipulations happen in compile time only.

## Slides
See more details about the project in [slides](https://docs.google.com/presentation/d/1wNkQVGbKwA35M7la8PyUXYIUZ-XWMQ_VggzGHvIQGPM/edit?slide=id.g38a05140cf9_0_72#slide=id.g38a05140cf9_0_72) from IWACO @ SPLASH 2025 

## Code
* Implementation of the Array API and Merge Sort: disjoint_mut_test
* Benchmarking: custom_benchmark

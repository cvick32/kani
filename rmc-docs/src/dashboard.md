# RMC Dashboard

The [RMC Dashboard](./dashboard/index.html) is a testing tool based on [Litani](https://github.com/awslabs/aws-build-accumulator).

The purpose of the dashboard to show the level of support in RMC for all Rust features.
To this end, we use Rust code snippet examples from the following general Rust documentation books:
 * The Rust Reference
 * The Rustonomicon
 * The Rust Unstable Book
 * Rust By Example

However, not all examples from these books are suited for verification.
For instance, some of them are only included to show what is valid Rust code (or what is not).

Because of that, we run up to three different types of jobs when generating the dashboard:
 * `check` jobs: This check uses the Rust front-end to detect if the example is valid Rust code.
 * `codegen` jobs: This check uses the RMC back-end to determine if we can generate GotoC code.
 * `verification` jobs: This check uses CBMC to obtain a verification result.

Note that these are incremental: A `verification` job depends on a previous `codegen` job.
Similary, a `codegen` job depends on a `check` job.

> **Warning:** [Litani](https://github.com/awslabs/aws-build-accumulator) does
> not support hierarchical views at the moment. For this reason, we are
> publishing a [text version of the dashboard](./dashboard/dashboard.txt) which
> displays the same results in a hierarchical way while we work on adding more
> features to [Litani](https://github.com/awslabs/aws-build-accumulator).

Before running the above mentioned jobs, we pre-process the examples to:
 1. Set the expected output according to flags present in the code snippet.
 2. Add any required compiler/RMC flags (e.g., CBMC unwinding flags).

Finally, we run all jobs, collect their outputs and compare them against the expected outputs.
The results are summarized as follows: If the obtained and expected outputs differ,
the color of the stage bar will be red. Otherwise, it will be blue.
If an example shows one red bar, it is considered a failed example that cannot be handled by RMC.

The [RMC Dashboard](./dashboard/index.html) and [its text version](./dashboard/dashboard.txt) are
automatically updated whenever a PR gets merged into RMC.

## The dashboard procedure

This section describes how the dashboard operates at a high level.

To kick off the dashboard process use

```
./x.py run -i --stage 1 dashboard
```

The main function of the dashboard is `generate_dashboard()` in
[`src/tools/dashboard/src/books.rs`](https://github.com/model-checking/rmc/blob/main/src/tools/dashboard/src/books.rs),
which follows these steps:
 * First, it calls the different `parse_..._hierarchy()` functions which parse
   the summary files for each book.
 * The `extract_examples(...)` function uses `rustdoc` to extract all examples
   from the books.
 * Then for each example it will check if there is a corresponding `.props` file
   in `src/tools/dashboard/configs/`. The contents of these files (e.g.,
   command-line options) are prepended to the example.
 * All examples are written in the `src/test/dashboard/books/` folder.

   In general, the path to a given example is
   `src/test/dashboard/books/<book>/<chapter>/<section>/<subsection>/<line>.rs`
   where `<line>` is the line number where the example appears in the
   documentation. The `.props` files mentioned above follow the same naming
   scheme in order to match them and detect conflicts.

 * Then all examples are run using
   [Litani](https://github.com/awslabs/aws-build-accumulator).
 * Finally, the Litani log is used to generate the [text version of the
   dashboard](./dashboard/dashboard.txt).

> **Warning:** Note that any changes done to the examples in
> `src/test/dashboard/books/` may be gone if the dashboard is executed.
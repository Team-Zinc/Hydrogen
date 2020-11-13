# Hydrogen
A build system that is nice to use.

## What is Hydrogen?
Glad you asked! Hydrogen is a build system... *for the modern world*! Hydrogen doesn't use an obvious intermediate stage, like a Makefile (like, ugh, CMake).
Instead, Hydrogen uses a internal struct!

## Examples
If you want more in-depth examples, please see the examples directory. Here is a quick one however!

`./Hydrogen.yml`
```yaml
name: greet
description: A collection of greeter programs
authors: [Milo Banks]
version: 1.0.0

type: Binary
language: C++
```

`./Build.yml`
```yaml
files: ['**.cpp', '**.hpp']
# OR
files: ['main.cpp']
```

Simple, right? It doesn't use a fancy `DSL` like CMake, or weird syntax like CMake.

One of the design philosophies about Hydrogen is to make it easy to guess. This means if you don't know the name of a field, you should be able to guess, and probably be right. CMake doesn't seem to follow this.

If you thought that was complicated, simply look at the corresponding CMake `DSL` code.

```rust
cmake_minimum_required(VERSION 2.8.9)
project(hello DESCRIPTION "A collection of greeter programs" VERSION 1.0.0)
add_executable(hello main.cpp)
```

Even though the CMake is shorter, you probably know that it's really easy to shoot yourself in the foot with it. Plus, it just look *hideous*.

## Vocab
Here is some vocab that will help you on your Hydrogen based journey!

### Actual
An "actual" is a thing that provides data on how to **actual**ly build a project. Hydrogen provides three types of actuals.

The first is the static actual. This is just a *yaml* file that gets translated almost directly into a real actual.

A "dynamic" actual is a *Python* script that is run to make changes to a real actual, or create one dynamically all together.

A "real" actual is the actual internal struct that Hydrogen uses to build your project. You never even see it (unless you want to).

### Hydrogen.yml
A "metadata" file that contains information about your project, at a glance!

### Fetch.yml
Provides information on how to download that part of the project.

### Build.yml
A static actual. By this, I mean an actual that cannot be changed, hence the name static.

### Build.py
A dynamic actual.

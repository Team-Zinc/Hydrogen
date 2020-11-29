# Hydrogen
A build system that is nice to use.

## What is Hydrogen?
Glad you asked! Hydrogen is a build system... *for the modern world*! Hydrogen doesn't use an obvious intermediate stage, like a Makefile (like, ugh, CMake).
Instead, Hydrogen uses a internal struct!

## Why did I feel the need to make *yet another* build system?
I was going to write a poem, and then I realized rhyming is hard. Who knew?
Anyways I wrote Hydrogen because....
CMake is like using a 100,000 dollar unconventional forestry tree removal machine with the instructions written is a mix of Mandarin, Arabic, and Finish to remove weeds from your 2 foot by 2 foot flower garden ([analogy gotten here](https://stackoverflow.com/a/58755686)).
Scons is basically telling a snail to cary out some instructions that are sometimes understandable.

## Examples
If you want more in-depth examples, please see the examples directory. Here is a quick one however!

`./Hydrogen.yml`
```yaml
name: greet
description: A collection of greeter programs
authors: [Milo Banks]
version: 1.0.0

type: Binary
```

`./Build.yml`
```yaml
files: ['**/*.cpp']
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

## Notes:
Before policies are implemented (probably in the next PR), you will need to make sure that if you want a project to build, you must supply a type. This is just because Hydrogen optimises out building projects that don't have a type (and are thus assumed just to be parent projects).

## Vocab
Here is some vocab that will help you on your Hydrogen based journey!

### Actual
An "actual" is a thing that provides data on how to **actual**ly build a project. Hydrogen provides three types of actuals.

The first is the static actual. This is just a *yaml* file that gets translated almost directly into a real actual.

A "dynamic" actual is a *Python* script that is run to make changes to a real actual, or create one dynamically all together.

A "real" actual is the actual internal struct that Hydrogen uses to build your project. You never even see it (unless you want to).

### Hydrogen.yml
A "metadata" file that contains information about your project, at a glance!

### Build.yml
A static actual. By this, I mean an actual that cannot be changed, hence the name static.

### Build.py
A dynamic actual.

## Order
Hydrogen parses and (possibly) runs the build files in a certain order. They are as follows:
```
2. Meta.yml
3. Build.yml
4. Build.py
```

First, you want to figure out information about the project.

After that, you want to create a static actual, and then possibly change it with a dynamic actual.

Please note that if any of these files are missing, they will just be skipped over. The only required file is `Meta.yml`.

## OS and Compiler Support
Hydrogen is developed on Linux systems, with a dash of Mac. As such, Hydrogen supports `gcc` and `clang` (plus their C++ counterparts). Hydrogen does **not** support MSVC, but this is easily worked around by either installing a linux subsystem, or a port like `mingw-w64`.
As for OS support, Hydrogen should work well on Linux and Mac, but Windows is a bit more iffy. As Hydrogen isn't tested that much on Windows (or at all for that matter), Hydrogen is unstable on Windows. However, everything should work just fine. If you have a Windows machine, and would like to contribute and solve a few issues, I beg you. Actually. *Please!*

## Philosophies
These are the philosophies of Hydrogen I really try and follow:
##### 1. It should be guessable. This means you should be able to guess the name of a field, and be right.
For example, if you forgot how to say where a dependency is located in the dependencies list in a build file, `at` seems a reasonable guess, and so `at` it is.

##### 2. It must not make you learn a new language (most people know Python, and yaml is pretty simple).
CMake doesn't follow this rule. It uses it's own weird DSL. Lots of people know Python (which makes it an ideal build system language for newcomers).

##### 3. It must work correctly.
It's that simple. It should work. And Hydrogen does. Except for a few cases, but hey I'm the only one working on Hydrogen, so give me a break.

##### 4. It must produce reproducible builds.
Now, this one is not so simple. You can either have builds (on different machines) that, when built, produce the *exact* same binary/bytecode.
This is a bit extreme, so instead, by this, I mean that the same code in, built the same way, should be enough. 

##### 5. It must look nice, but not to the point where it infringes opon the other rules.
Its simple. It must look nice. Hydrogen takes inspiration from [yarn](yarnpkg.com) in this regard.

##### 6. It must be dependency based. This means that its easy to pull in another project from Open Source land.
This is what Hydrogen was made for. Getting dependencies, and building them nicely. You don't need to have dependencies in a fancy shmanshy HUR (Hydrogen User Repositories). You can just tell it to get it from Github, or a regular git repo, or just a tarball. It should be able to get dependencies that aren't built with itself.

##### 7. It must be pretty fast.
Simple stuff. Each build task in Hydrogen that runs concurrently also has a configuration part, that accesses a pool. If the building process tells the configuration process that it needs header `y`, and the configuration pool says nothing about header `y`, then Hydrogen should look for it, without holding up the rest of the builds. This makes Hydrogen fast.

##### 8. It must have maintainable code.
By this, I don't mean the projects that are built with Hydrogen must be maintainable. I mean Hydrogen itself must be maintainable. I've taken steps so Hydrogen is.

##### 9. It should make project builds as independent as possible.
Isolated builds mean that one build shouldn’t change because of another build. Hydrogen doesn't tell you not to do it, but it's setup in a way that its hard to.

##### 10. It must cache state between builds.
Hydrogen does this with a single lockfile, unlike CMake which uses an entire directory and more!

##### 11. It must make build iteration easy.
Hydrogen does this. You just tell it to build over there, and get back to you. And it does that.

##### 12. It must not get in the way of development.
CMake definitely doesn't follow the last part of this rule. You shouldn't have to follow a certain structure for your project to work, nor must the build files be hard to write. 

##### 13. Builds should parallelize along a dimension that increases with code size.
Hydrogen runs multiple builds and configuration steps in parallel.

##### 14. A project must be able to be built from any commit (a.k.a no build files outside the source tree).
This just means that the build system shouldn't touch anywhere outside the repository for anything build related (logs are obviously fine).
##### 15. Changes to the build files should be tracked and versioned.
Without being able to version build configuration, it’s difficult to track when and who made a change to the build  Because changes to the build system can drastically change the resulting build, this information is important for reproducible builds. Hydrogen makes this easy with everything inside the repository, and a built in timeline.

##### 16. It should be easy to set up.
This compliments #1. If you have a general idea of Hydrogen, and have used it before, but forgot most of it, then you can still set up a project, because of, again, #1.

##### 17. It must be easy to expand.
By this I mean that you should be able to support more compilers, languages, etc... without having to change the frontend.
Hydrogen does this by splitting each backend into its own directory.
##### 18. A developer should not have to extort a bug in the build system to achieve something.
CMake is notorious for this. If there is a function you want to get some functionally out of it, but it doesn't work at all like you expected, then you can either spends hours writing your own function.... or simply extort a bug.
Hydrogen doesn't like this, which means we have to make the build system flexible.
##### 19. It should not have an obvious intermediate stage.
Unlike pretty much every build system for C/C++ out there, it generates build files (such as Make, Ninja, MSBuild, etc...). This gives the build system less control over the building process, and is generally a bit slower because you waste time on build file generation and parsing.

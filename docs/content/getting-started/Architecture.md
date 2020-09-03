---
title: "Architecture"
date: 2020-09-02T23:50:08-06:00
draft: true
weight: 2
---

### Architecture
How the f*ck does Hydrogen work?

#### Configuration
Hydrogen works by reading in a ```YAML``` configuration file, which serves as a blueprint of sorts for the project. From specifying dependencies, files, configurations, it specifies it all.
But what if I need to check something at configure time that Hydrogen doesn't do for me? Well, simply provide a hook! Hooks are pivotal to Hydrogen. You can supply a configuration hook, and it will run code at configuration time. You can find more hooks in the documentation.
But what code does it execute? Good question! Hydrogen uses the Lua scripting language to make extending Hydrogen easy as pie (well, pie is actually kind of hard to make, and programming and backing are diferent things, but you get the point).

#### Building
Unlike other build tools/systems (such as Premake and CMake), Hydrogen doesn't actually shove out a Makefile or a IDE project file; instead, it creates a blueprint for your *entire* project (yes, this includes dependencies). Hydrogen then uses this to build dependencies on their own from the farthest out to the closest to source concurrently. This means *blazing* fast compile times, as dependencies are compiling in paralel.

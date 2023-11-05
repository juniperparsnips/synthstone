# Crafter Controller Instructions

This document outlines the assembly instructions needed to run the MVP of the crafting computer.

## Instructions Types
Instructions are divided into two main types: "R" and "I".
R type instructions are those whose parameters are interpreted as memory addresses within RAM.
I type instructions are those whose parameters are taken directly as static values.

## Instruction List

### craft (R)
Send a craft request to a crafting core: 
`craft <recipe_id> <n> <out>`

where:
* **recipe_id**: the address in RAM storing the recipe ID to use
* **n**: the address in RAM storing the number of crafting operations to perform
* **out**: the address in RAM storing the output location of the crafting request 

### craft_i (I)
Send a craft request to a crafting core (see [craft (R)](#craft (R)))

### poll_st (R)
Poll the status of a storage system request:
`poll_st <id> <status> <info>`

Where:
* **id**: the address in RAM storing the storage request ID to poll
* **status**: the address in RAM where the request's status code should be written
* **info**: the address in RAM where the status details (i.e. quantity defecit) should be written

Status codes are:
* **0**: Request complete (info doesn't matter)
* **1**: Request in progress (info doesn't matter)
* **2**: Failed--Not enough items (info stores the defecit in items)

### poll_cc (R)
Poll the status of a crafting core:
`poll_cc <id> <status> <info>`

Where:
* **id**: the address in RAM storing the crafting core ID to poll
* **status**: the address in RAM where the request's status code should be written
* **info**: the address in RAM where the status details should be written

Status codes are:
* **0**: OK -- Can take new requests
* **1**: FULL -- Can not take new requests (i.e. all register blocks within the crafting core are full)

### jmpc (I)
Conditionally jump the program counter:
`jmpc <c> <addr>`

Sets the program counter to `<addr>` if the value in `<c>` is non-0

Where:
* **c**: the address in RAM holding the condition
* **addr**: the address to be jumped to

### add (R)
Add two numbers:
`add <a> <b> <t>`

Writes the result of `a + b` to `t`

Where:
* **a**: the address in RAM storing the first addend
* **b**: the address in RAM storing the second addend
* **t**: the address in RAM where the sum should be stored

### add_i (I)
Add two numbers (see [add (R)](#add (R))) however `t` is still an address in RAM.

### sub (R)
Subtract two numbers:
`sub <a> <b> <t>`

Writes the result of `a - b` to `t`

Where:
* **a**: the address in RAM storing the minuend (first term)
* **b**: the address in RAM storing the subtrahend (second term)
* **t**: the address in RAM where the difference should be stored

### sub_i (I)
Subtract two numbers (see [sub (R)](#sub (R))) however `t` is still an address in RAM.

### ld (I)
Load the value into memory:
`ld <value> <t>`

Loads the given value into memory address `a`

Where:
* **a**: the address in RAM to load to
* **value**: the immediate value to be written

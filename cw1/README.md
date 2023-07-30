# Memory Management

## Introduction

Programming languages and systems have traditionally used one of three approaches to managing memory: the system either provides a garbage collector, or it supports reference counting with automatic memory reclamation when the reference count reaches zerio, or it relies on the programmer to manually allocate and free memory. There are advantages and disadvantages to each, but the fact that all three are widely used suggests that none of them are suitable for all problem domains.

A new automatic memory management scheme that is gaining attention is *region-based memory management*. This is used in the [Rust programming language](https://rust-lang.org/), and is based on ideas developed in the older [Cyclone research language](http://cyclone.thelanguage.org/). Region-based memory management tries to provide effective automatic memory management, without the indeterminism and overheads of a garbage collector, by tracking ownership of data and using this understanding of ownership to automatically deallocate objects when they go out of scope.

**In this assessed exercise**, region-based memory management will be studied, discussing its advantages and problems when compared to other approaches to memory management.

## Assessed Exercise 1

There are three parts to this exercise. A single report should be prepared that includes answers to all three parts.

**In part one**, research and prepare a description of how memory management works in the Rust programming language, and how the Rust language manages ownership of data, including different pointer types and borrowed references to data. Describe when memory is allocated and deallocated in Rust programs, how Rust tracks ownership and borrowing of data and how this is controlled via restrictions on references, and what safety guarantees result from this control. [10 marks]

**In part two**, compare and contrast how memory management works in Rust with memory management in the C programming language. Discuss how these two languages compare in terms of 1) programmer effort to manage memory; 2) runtime efficiency; and 3) safety and flexibility. [10 marks]

**In part three**, compare and contrast region-based memory management using ownership tracking, as used by Rust, with memory management in garbage collected languages. Highlight the relative advantages and disadvantages of the approach used in Rust as compared to a garbage collected language. *With the aid of sample code fragments*, discuss both the types of program that Rust makes easy to write, and the types of program that are difficult, or impossible, to write in Rust. Explain what language design decisions make such programs easy or difficult to write. [10 marks]

## Submission

Submit a single report, in PDF format, containing the answers to the three parts of the assessed exercise. A mark out of 30 will be assigned to the submission, weighted as noted earlier. This mark will be converted to a percentage, then used to assign a band on the University's 22 point scale.

Prepare the PDF file formatted for A4 paper, in two columns, using the Times Roman font in 10pt, with 1.5cm margins, and the exercise title, GUID, and the gate at the top (i.e., using a format similar to the handout). If using LaTeX to prepare the report, the `.tex` file should start like the following:

```tex
\documentclass[10pt,a4paper,twocolumn]{article}
\usepackage[cm]{fullpage}
\usepackage{newtxtext}
\usepackage{newtxmath}
\begin{document}
\title{Advanced Systems Programming (H/M) Exercise 1}
\author{1234567a} % Replace with GUID
\date{\today}
\maketitle
\section*{Part 1}
```

LaTeX does not need to be used. The report must not exceed four pages in length, including all figures, tables, code samples, and references. Length is not an indication of merit: if the required material can be covered in less than four pages, then do so.

Submit your report before 10:00am on 20 February 2023. Following the code of assessment, late submissions will be accepted for up to 5 working days beyond this due date. Late submissions will receive a two band penalty for each working day, or part thereof, the submission is late. Submissions that are received more than five working days after the due date will be awarded a band of H.

Submissions must be made via Moodle. This exercise is worth 10% of the mark for this course. Submit a single PDF file entitled `asp-ex1-GUID.pdf`. Submissions that do not following submission instructions will be given a two band penalty. Penalties will be strictly enforced.

<!-- If any illness or other circumstances affect your submission, then contact the course coordination *before* the deadline to require an extension, following the usual procedure. -->

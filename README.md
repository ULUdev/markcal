# `markcal`
## A (very) simple program to manage your marks/grades (if they are represented numerically)

This program can manage your marks (for the most part) by subject and give you
the average mark either per subject or as a complete report. This program in
it's initial form was written in like an hour so don't expect the most advanced
things yet.

Marks are saved into a csv-like format and you can find them by default in
`~/.markcal`. You can also specify a custom file with `-f`. Marks are saved as follows:
```
<subject>,<grade>,<class>[,<comment>]
```
The subject is a string (I tested with umlauts and it worked so it should
probably work with any unicode character). The grade is an integer and the
comment is a string again that can contain commas. Class refers to the
grade-class. Taken from the german system: You get different kinds (or classes)
of grades. In germany this is exams and others. First all exam marks are
averaged and all others are averaged and then these two resulting averages are
averaged again.

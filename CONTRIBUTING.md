Thank you for your interest in contributing! To make our work smooth, please
consider these guidelines while preparing your changes:

- The repository tries to follow the [PEP 8] Python style guide. However, please exercise
  your judgement. Justified deviations are allowed.
  - In particular, the maximum line length is only a lax guideline. Don't break your
    code into separate lines if there isn't a reasonable place to put the linebreak.
  - The nature of the project means
  - Also, don't submit formatting adjustments inside files you didn't otherwise modify.
    Clearly, the current formatting was considered good enough when the file was
    last modified.
- Make commit messages descriptive.
- Keep commits focused on one thing. As a *guideline*, if your commit message subject
  contains the word *and*, you *might* want to split the commit into two.
- When moving big blocks of code between files, do so in a separate commit. This
  is necessary because, as far as I am aware, there are no tools that visualize
  such changes suitably (i. e. not as separate deletions and additions).
  The method for reviewing such changes I settled on is to reproduce the commit
  with a text editor. This works best when code movements are in a separate commit.

[PEP 8]: https://www.python.org/dev/peps/pep-0008/

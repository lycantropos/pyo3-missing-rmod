In what follows `python` is an alias for `python3.6` or `pypy3.6`
or any later version (`python3.7`, `pypy3.7` and so on).

Steps to reproduce
------------------
Install the latest `pip` & `setuptools` packages versions
```bash
python -m pip install --upgrade pip setuptools
```
Build library
```bash
python setup.py develop
```
Open python REPL
```bash
python
```
Execute following statements
```python
>>> from pyo3_missing_rmod import A, B
>>> A() % B()
```
Expected output:
```python
A.__mod__ is called
B.__rmod__ is called
Traceback (most recent call last):
  File "<input>", line 1, in <module>
TypeError: unsupported operand type(s) for %: 'A' and 'B'
```
Actual output:
```python
A.__mod__ is called
Traceback (most recent call last):
  File "<input>", line 1, in <module>
TypeError: 'A' object cannot be converted to 'B'
```

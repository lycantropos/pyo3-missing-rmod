from pathlib import Path
from typing import (TYPE_CHECKING,
                    Iterator)

from setuptools import (find_packages,
                        setup)

if TYPE_CHECKING:
    from setuptools_rust import RustExtension


class LazyRustExtensions:
    def __iter__(self) -> Iterator['RustExtension']:
        from setuptools_rust import RustExtension
        yield RustExtension('pyo3_missing_rmod')


setup(name='pyo3_missing_rmod',
      packages=find_packages(exclude=('tests', 'tests.*')),
      author='Azat Ibrakov',
      author_email='azatibrakov@gmail.com',
      license='MIT License',
      python_requires='>=3.6',
      setup_requires=Path('requirements-setup.txt').read_text('utf-8'),
      rust_extensions=LazyRustExtensions(),
      include_package_data=True,
      zip_safe=False)

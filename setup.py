from __future__ import annotations

from setuptools import setup


COMMITTED_VERSION = '1.1.5'


setup(
    name='pre_commit_placeholder_package',
    version='0.0.0',
    install_requires=[f'committed=={COMMITTED_VERSION}'],
    package_dir={'': 'crates'},
)

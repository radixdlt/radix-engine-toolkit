import setuptools

setuptools.setup(
    name="radix-engine-toolkit",
    version="0.12.1",
    packages=["radix_engine_toolkit"],
    author="radixdlt",
    description="A Python wrapper around the Radix Engine Toolkit that provides Radix Ledger primitives to Python.",
    long_description="Long Description",
    long_description_content_type="text/markdown",
    license='Apache-2.0',
    python_requires='>=3.6',
    install_requires=[],
    url="https://github.com/radixdlt/radix-engine-toolkit",
    project_urls={
        "Bug Tracker": "https://github.com/radixdlt/radix-engine-toolkit/issues",
    },
    classifiers=[
        "Programming Language :: Python :: 3",
        "Programming Language :: Python :: 3.7",
        "Programming Language :: Python :: 3.8",
        "Programming Language :: Python :: 3.9",
        "Programming Language :: Python :: 3.10",
        "Programming Language :: Python :: 3.11",
        "Natural Language :: English",
        "Operating System :: OS Independent",
        "License :: OSI Approved :: MIT License",
    ],
)
# fabrik

Implementation of
[FABRIK](http://www.andreasaristidou.com/FABRIK.html) (Forward And
Backward Reaching Inverse Kinematics) in Rust.

Closely follows Python implementation at [saleone/pyfabrik](https://github.com/saleone/pyfabrik).

## Goal
![Inverse kinematics example with human skeleton.](http://www.andreasaristidou.com/publications/images/FABRIC_gif_1.gif)

## Roadmap

- [x] Basic 2D (flat chain)
- [ ] Basic 3D (flat chain)
- [ ] 3D testing sandbox
- [ ] Basic 2D joint movement restrictions
- [ ] Basic 3D joint movement restrictions
- [ ] Complex chain support 2D
- [ ] Complex chain support 3D

## Contributing

__All contributions are appreciated.__

Read the paper [paper](http://www.andreasaristidou.com/publications/papers/FABRIK.pdf).

FABRIKs [homepage](http://www.andreasaristidou.com/FABRIK.html) has links to other implementations.

## [License](./LICENSE)

MIT License

Copyright (c) 2021 Saša Savić

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

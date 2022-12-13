#pragma once

#include <iostream>

struct Point {
    int x;
    int y;
};

class Rect {
public:
    Point top_left;
    Point bottom_right;

    int width() const { return bottom_right.x - top_left.x; }

    int height() const { return bottom_right.y - top_left.y; }
};

inline void print_point(Point p) {
    std::cout << "(" << p.x << ", " << p.y << ")\n";
}


class MessageBuffer {
public:
    // std::string is not a trivial type because in some STL implementations
    // it may contain a self-referential pointer.
    void add_blurb(std::string blurb) {
        message += blurb;
    }

    std::string get() const {
        return message;
    }

private:
    std::string message;
};
#include <di/prelude.h>
#include <dius/prelude.h>
#include <fcntl.h>
#include <stdio.h>
#include <unistd.h>

auto part_a(di::StringView input) {
    auto numbers = input | di::split(U'\n') | di::transform(di::parse_unchecked<i32>) | di::to<di::LinkedList>();

    auto cycled = numbers | di::cycle;

    auto iter = cycled.begin();
    auto og_order = di::range(numbers.size()) | di::transform([&](auto) {
                        return iter++;
                    }) |
                    di::to<di::Vector>();

    for (auto node : og_order) {
        auto times = *node;
        if (times > 0) {
            times %= (numbers.size() - 1);
        } else {
            times = -(di::abs(times) % (numbers.size() - 1));
        }

        if (times >= 0) {
            ++times;
        }

        auto position = di::next(node, times);
        numbers.splice(position.base(), numbers, node.base());
    }

    auto zero = di::find(cycled, 0);
    auto it = di::next(zero, 1000);
    auto jt = di::next(it, 1000);
    auto kt = di::next(jt, 1000);

    return *it + *jt + *kt;
}

auto part_b(di::StringView input) {
    auto numbers = input | di::split(U'\n') | di::transform(di::parse_unchecked<i64>) | di::transform([](auto i) {
                       return i * 811589153;
                   }) |
                   di::to<di::LinkedList>();

    auto cycled = numbers | di::cycle;

    auto iter = cycled.begin();
    auto og_order = di::range(numbers.size()) | di::transform([&](auto) {
                        return iter++;
                    }) |
                    di::to<di::Vector>();

    for (auto i : di::range(10)) {
        (void) i;
        for (auto node : og_order) {
            auto times = *node;
            if (times > 0) {
                times %= (numbers.size() - 1);
            } else {
                times = -(di::abs(times) % (numbers.size() - 1));
            }

            if (times >= 0) {
                ++times;
            }

            auto position = di::next(node, times);
            numbers.splice(position.base(), numbers, node.base());
        }
    }

    auto zero = di::find(cycled, 0);
    auto it = di::next(zero, 1000);
    auto jt = di::next(it, 1000);
    auto kt = di::next(jt, 1000);

    return *it + *jt + *kt;
}

int main() {
    auto buffer = di::Vector<c8> {};

    int fd = open("input.txt", O_RDONLY);
    buffer.reserve(BUFSIZ);

    ssize_t result;
    while ((result = read(fd, buffer.data() + buffer.capacity() - BUFSIZ, BUFSIZ)) > 0) {
        buffer.assume_size(buffer.size() + result);
        buffer.reserve(buffer.size() + BUFSIZ);
    }

    close(fd);

    auto string = di::create<di::String>(di::move(buffer));

    dius::debug_log("Part A: {}"_sv, part_a(string));
    dius::debug_log("Part B: {}"_sv, part_b(string));
}
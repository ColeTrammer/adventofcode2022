#include <di/prelude.h>
#include <fcntl.h>
#include <stdio.h>
#include <unistd.h>

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

    {
        auto numbers = string | di::split(U'\n') | di::transform(di::parse_unchecked<i32>) | di::to<di::Vector>();

        struct Node {
            Node(int v, Node* n = nullptr, Node* p = nullptr) : value(v), next(n), prev(p) {}

            int value { 0 };
            Node* next { nullptr };
            Node* prev { nullptr };
            Node* n { nullptr };
            bool visited { false };
        };

        Node _head(0);
        Node* head = &_head;
        Node* tail = head;
        Node* rh = nullptr;

        for (auto x : numbers) {
            auto y = new Node(x, head, tail);
            tail->next = y;
            head->prev = y;
            tail = y;

            if (rh == nullptr) {
                rh = y;
            } else {
                rh->n = y;
                rh = y;
            }
        }

        auto sz = numbers.size();

        for (auto it = head->next; it != head;) {
            if (it->visited) {
                it = it->next;
                continue;
            }

            auto to_rotate = it->value;
            if (to_rotate > 0) {
                to_rotate %= (sz - 1);

                auto p = it->next;
                for (; to_rotate || it->next == head; --to_rotate) {
                    auto* to_swap = it->next;
                    if (to_swap == head) {
                        to_rotate++;
                    }

                    it->prev->next = to_swap;
                    to_swap->next->prev = it;

                    to_swap->prev = it->prev;
                    it->prev = to_swap;
                    it->next = to_swap->next;
                    to_swap->next = it;
                    it->visited = true;
                }
                it = p;
            } else {
                to_rotate = -to_rotate % (sz - 1);

                auto* p = it->next;
                for (; to_rotate || it->prev == head; --to_rotate) {
                    auto* to_swap = it->prev;
                    if (to_swap == head) {
                        to_rotate++;
                    }

                    to_swap->prev->next = it;
                    it->next->prev = to_swap;

                    it->prev = to_swap->prev;
                    to_swap->prev = it;
                    to_swap->next = it->next;
                    it->next = to_swap;
                    it->visited = true;
                }

                it = p;
            }

            // auto res = di::Vector<int> {};
            // for (auto it = head->next; it != head; it = it->next) {
            //     res.push_back(it->value);
            // }

            // for (auto x : res) {
            //     printf("%d ", x);
            // }
            // printf("\n");
        }

        auto res = di::Vector<int> {};
        for (auto it = head->next; it != head; it = it->next) {
            res.push_back(it->value);
        }

        auto it = di::find(res, 0);
        auto index = (it - res.begin());

        auto a = (index + 1000) % res.size();
        auto b = (index + 2000) % res.size();
        auto c = (index + 3000) % res.size();

        printf("A: %d [%d + %d + %d]\n", res[a] + res[b] + res[c], res[a], res[b], res[c]);
    }
    {
        i64 key = 811589153;
        auto numbers = string | di::split(U'\n') | di::transform(di::parse_unchecked<i64>) | di::transform([&](auto x) {
                           return x * key;
                       }) |
                       di::to<di::Vector<i64>>();

        struct Node {
            Node(i64 v, Node* n = nullptr, Node* p = nullptr) : value(v), next(n), prev(p) {}

            i64 value { 0 };
            Node* next { nullptr };
            Node* prev { nullptr };
            Node* n { nullptr };
            bool visited { false };
        };

        Node _head(0);
        Node* head = &_head;
        Node* tail = head;
        Node* rh = nullptr;
        Node* rt = nullptr;

        for (auto x : numbers) {
            auto y = new Node(x, head, tail);
            tail->next = y;
            head->prev = y;
            tail = y;

            if (rh == nullptr) {
                rh = rt = y;
            } else {
                rt->n = y;
                rt = y;
            }
        }

        auto sz = numbers.size();

        for (auto i : di::range(10)) {
            (void) i;
            auto res = di::Vector<i64> {};
            for (auto it = head->next; it != head; it = it->next) {
                res.push_back(it->value);
            }

            for (auto it = rh; it; it = it->n) {
                auto to_rotate = it->value;
                if (to_rotate > 0) {
                    to_rotate %= (sz - 1);

                    for (; to_rotate || it->next == head; --to_rotate) {
                        auto* to_swap = it->next;
                        if (to_swap == head) {
                            to_rotate++;
                        }

                        it->prev->next = to_swap;
                        to_swap->next->prev = it;

                        to_swap->prev = it->prev;
                        it->prev = to_swap;
                        it->next = to_swap->next;
                        to_swap->next = it;
                        it->visited = true;
                    }
                } else {
                    to_rotate = -to_rotate % (sz - 1);

                    for (; to_rotate || it->prev == head; --to_rotate) {
                        auto* to_swap = it->prev;
                        if (to_swap == head) {
                            to_rotate++;
                        }

                        to_swap->prev->next = it;
                        it->next->prev = to_swap;

                        it->prev = to_swap->prev;
                        to_swap->prev = it;
                        to_swap->next = it->next;
                        it->next = to_swap;
                        it->visited = true;
                    }
                }
            }
        }

        auto res = di::Vector<i64> {};
        for (auto it = head->next; it != head; it = it->next) {
            res.push_back(it->value);
        }

        auto it = di::find(res, 0);
        auto index = (it - res.begin());

        auto a = (index + 1000) % res.size();
        auto b = (index + 2000) % res.size();
        auto c = (index + 3000) % res.size();

        printf("B: %ld [%ld + %ld + %ld]\n", res[a] + res[b] + res[c], res[a], res[b], res[c]);
    }
}
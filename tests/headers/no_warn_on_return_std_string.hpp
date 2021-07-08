// bindgen-flags: --opaque-type "std::.+" --enable-cxx-namespaces --allowlist-type "StdStringReturnedByValue"

namespace std {
template <typename> class allocator;
template <class> struct char_traits;
template <typename a, typename = char_traits<a>, typename = allocator<a>>
class basic_string;
typedef basic_string<char> string;
template <typename> class allocator {};
template <typename, typename, typename> class basic_string {
public:
  struct {};
  long b;
  enum { c = 15 };
  char d[c];
  basic_string(const char *, const allocator<char> & = allocator<char>());
};
} // namespace std

struct StdStringReturnedByValue {
  std::string value();
};
std::string StdStringReturnedByValue::value() {
  std::string value("hello");
  return value;
}

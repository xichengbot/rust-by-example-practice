chapters=(
"ch02_primitives"
"ch03_custom_types"
"ch04_variable_bindings"
"ch05_types"
"ch06_conversion"
"ch07_expressions"
"ch10_modules"
"ch11_crates"
"ch12_cargo"
"ch13_attributes"
"ch16_traits"
"ch17_macros"
"ch18_error_handling"
"ch19_std_library_types"
"ch20_std_misc"
"ch21_testing"
"ch22_unsafe_operations"
"ch23_compatibility"
"ch24_meta"
)

for ch in "${chapters[@]}"; do
  if [ ! -d "$ch" ]; then
cargo new "$ch"
else
echo "✅ 目录 $ch 已存在，跳过..."
fi
done

#!/bin/bash
# demo_unicode.sh - Unicode 数学符号演示脚本

echo "======================================"
echo "  Unicode 数学符号计算器演示"
echo "======================================"
echo ""

# 颜色定义
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 函数：运行表达式并显示结果
run_expr() {
    local expr="$1"
    local desc="$2"

    echo -e "${BLUE}表达式:${NC} $expr"
    if [ ! -z "$desc" ]; then
        echo -e "${YELLOW}说明:${NC} $desc"
    fi

    result=$(echo "$expr" | cargo run -p chapter02 --example expression_calculator 2>/dev/null | grep "^=" | sed 's/^= //')

    if [ ! -z "$result" ]; then
        echo -e "${GREEN}结果:${NC} $result"
    else
        echo -e "${YELLOW}(运行表达式...)${NC}"
        echo "$expr
quit" | cargo run -p chapter02 --example expression_calculator 2>&1 | grep -A 1 "^>" | tail -1 | sed 's/^= /结果: /'
    fi
    echo ""
}

echo "======================================"
echo "1. 基本 Unicode 运算符"
echo "======================================"
echo ""

run_expr "5 × 3" "使用 × 代替 *"
run_expr "20 ÷ 4" "使用 ÷ 代替 /"
run_expr "10 × 3 ÷ 2" "混合使用"

echo "======================================"
echo "2. 平方根符号 √"
echo "======================================"
echo ""

run_expr "√16" "简单平方根"
run_expr "√144" "更大的数"
run_expr "√(16 + 9)" "带括号的表达式"
run_expr "2 + √16 × 3" "混合运算"

echo "======================================"
echo "3. 幂运算上标"
echo "======================================"
echo ""

run_expr "5²" "5的平方"
run_expr "3³" "3的立方"
run_expr "2² + 3²" "勾股定理的一部分"
run_expr "10²" "100"

echo "======================================"
echo "4. 圆周率 π"
echo "======================================"
echo ""

run_expr "π" "圆周率的值"
run_expr "2 × π" "2π"
run_expr "π²" "π的平方"
run_expr "2 × π × 5" "圆的周长 (r=5)"

echo "======================================"
echo "5. 组合应用"
echo "======================================"
echo ""

run_expr "π × 5²" "圆的面积 (半径=5)"
run_expr "√(3² + 4²)" "勾股定理: √(9+16)"
run_expr "(4 ÷ 3) × π × 3³" "球体体积 (半径=3)"
run_expr "√100 × π" "组合运算"

echo "======================================"
echo "6. 复杂表达式"
echo "======================================"
echo ""

run_expr "2 × π × √(5² + 12²)" "复杂几何计算"
run_expr "(√16 + √9) × π" "多个根号"
run_expr "5² - 4 × 1 × 6" "二次方程判别式"

echo "======================================"
echo "7. 对比：Unicode vs ASCII"
echo "======================================"
echo ""

echo -e "${BLUE}Unicode 形式:${NC}"
run_expr "√16 + π × 5²"

echo -e "${BLUE}等价的 ASCII 形式:${NC}"
run_expr "sqrt(16) + 3.14159265359 * (5 ^ 2)"

echo "======================================"
echo "演示完成！"
echo "======================================"
echo ""
echo "提示："
echo "  - 运行交互式计算器: cargo run -p chapter02 --example expression_calculator"
echo "  - 查看文档: cat chapter02/UNICODE_MATH_SYMBOLS.md"
echo "  - 运行测试: cargo test -p chapter02 --lib unicode"
echo ""

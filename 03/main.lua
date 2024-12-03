local input = "/home/Nightmare/GitHub/AdventOfCode/03/input.txt"
local file = io.open(input, "r")

function getSource()
    if file then
        local content = file:read("*a")
        return content
    else
        print("failed to open")
    end
end

function extract_mul_calls(text)
    local pattern = "mul%(%d%d?%d?,%d%d?%d?%)"
    local result = {}
    for match in string.gmatch(text, pattern) do
        table.insert(result, match)
    end
    return table.concat(result, " ")
end

function parse_mul_string(input_string)
    local list_x = {}
    local list_y = {}

    for x, y in string.gmatch(input_string, "mul%((%d+),(%d+)%)") do
        table.insert(list_x, tonumber(x)) 
        table.insert(list_y, tonumber(y))
    end

    
    return {list_x, list_y}
end

function multiplyTables(input_table)
    local result = {}
    for i, v in ipairs(input_table[1]) do
        table.insert(result,input_table[1][i]*input_table[2][i])
    end
    return result
end

function SumTable(input_table)
    result = 0
    for i, v in pairs(input_table) do
        result = result+v
        
    end
    return result
end


function main()
    local source = getSource()
    local uncorrupted = extract_mul_calls(source)
    local MulTables = parse_mul_string(uncorrupted)
    local multipliedTable = multiplyTables(MulTables)
    print("result 1:")
    print(SumTable(multipliedTable))
end

main()
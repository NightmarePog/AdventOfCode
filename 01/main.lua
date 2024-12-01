local input = "C:\\Users\\Nightmare\\Desktop\\AdventOfCode\\01\\input.txt"
local file = io.open(input, "r")

function getSource()
    local buffer = {}
    if file then
        local content = file:lines()
        for line in content do
            table.insert(buffer, line)
        end
        return buffer
    else
        print("failed to open")
    end
end

function RowsToTable(input)
    local row1 = {}
    local row2 = {}
    for i, v in pairs(input) do
        local num1, num2 = v:match("^(%d+)%s+(%d+)$")
        table.insert(row1, tonumber(num1))
        table.insert(row2, tonumber(num2))
    end
    return {row1, row2}
end

function sort(input)
    for i, v in pairs(input) do
        for j, k in pairs(v) do
            table.sort(v)
        end
    end
   return input 
end

function tableDiffrence(input)
    table1 = input[1]
    table2 = input[2]
    result = {}
    for i = 1, #table1 do
        table.insert(result, math.abs(table1[i]-table2[i]))
    end
    return result
end

function TableSum(input)
    local result = 0
    for i, v in pairs(input) do
        result = result+v
    end
    return result
end

function checkDuplicates(list)
    local seen = {}
    for _, value in ipairs(list) do
        if seen[value] then
            return true
        end
        seen[value] = true
    end
    return false
end

function TableFind(table, value)
    local count = 0
    for _, v in ipairs(table) do
        if v == value then
            count = count + 1
        end
    end
    return count
end

function main()
    local sortedTable = sort(RowsToTable(getSource()))
    print("part1: ")
    print(
    TableSum(
    tableDiffrence(sortedTable)))

    print("part2: ")
    local buffer = {}
    for i, v in ipairs(sortedTable[1]) do 
        table.insert(buffer,TableFind(sortedTable[2],v)*v)
    end
    print(TableSum(buffer))
end

main()
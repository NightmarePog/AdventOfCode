local input = "/home/Nightmare/GitHub/AdventOfCode/02/input.txt"
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

function TableOfNumbers(tableinput)
    local MainTable = {}
    for i, v in ipairs(tableinput) do
        local result = {}
        for word in v:gmatch("%S+") do
            table.insert(result, tonumber(word))
        end
        table.insert(MainTable, result)
    end  
    return MainTable
end

function is_safe_row(row)
    local increasing = true
    local decreasing = true

    for i = 1, #row - 1 do
        local diff = row[i+1] - row[i]
        if diff > 0 then
            decreasing = false
        elseif diff < 0 then
            increasing = false
        end

        if math.abs(diff) > 3 or math.abs(diff) < 1 then
            return false
        end
    end

    return increasing or decreasing
end

function count_safe_rows(data)
    local count = 0
    for _, row in ipairs(data) do
        if is_safe_row(row) then
            count = count + 1
        end
    end
    return count
end

function is_safe_with_one_removal(row)
    if is_safe_row(row) then
        return true
    end

    for i = 1, #row do
        local modified_row = {}
        for j = 1, #row do
            if j ~= i then
                table.insert(modified_row, row[j])
            end
        end

        if is_safe_row(modified_row) then
            return true
        end
    end

    return false
end

function count_safe_reports(data)
    local count = 0
    for _, row in ipairs(data) do
        if is_safe_with_one_removal(row) then
            count = count + 1
        end
    end
    return count
end


function main()
    local source = getSource()
    local formatedTable = TableOfNumbers(source)
    local result = count_safe_rows(formatedTable)
    print("result 1:")
    print(result)
    local result2 = count_safe_reports(formatedTable)
    print("result 2:")
    print(result2)
end

main()
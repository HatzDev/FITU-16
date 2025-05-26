
function _init()
    printdb("Hello Lua World!")
end

function _draw()
    cls(0)
    line(
        {x = 0, y = 9},
        {x = 240, y = 9},
        10
    )
    line(
        {x = 0, y = 126},
        {x = 240, y = 126},
        1
    )
end
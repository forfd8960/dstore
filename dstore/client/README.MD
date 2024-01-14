## Run Hash Commands

```
>>>Set date 2024-01-14
done write cmd: [18, 20, 10, 18, 10, 4, 100, 97, 116, 101, 18, 10, 50, 48, 50, 52, 45, 48, 49, 45, 49, 52]
Not found for key: date
>>>Get date
done write cmd: [10, 6, 10, 4, 100, 97, 116, 101]
date: 2024-01-14

>>>Set person name:Alex|age:28|salary:5k
done write cmd: [18, 38, 10, 36, 10, 6, 112, 101, 114, 115, 111, 110, 18, 26, 110, 97, 109, 101, 58, 65, 108, 101, 120, 124, 97, 103, 101, 58, 50, 56, 124, 115, 97, 108, 97, 114, 121, 58, 53, 107]
Not found for key: person
>>>Get person
done write cmd: [10, 8, 10, 6, 112, 101, 114, 115, 111, 110]
person: name:Alex|age:28|salary:5k
```

## Run Set Commands

```
>>>SAdd myset e1:1 e2:2 e6:6
done write cmd: [42, 25, 10, 5, 109, 121, 115, 101, 116, 18, 4, 101, 49, 58, 49, 18, 4, 101, 50, 58, 50, 18, 4, 101, 54, 58, 54]
3
>>>SMembers myset
done write cmd: [50, 7, 10, 5, 109, 121, 115, 101, 116]
e1:1
e6:6
e2:2

>>>Scard myset
done write cmd: [58, 7, 10, 5, 109, 121, 115, 101, 116]
3
```

## Run List Commands

```
>>>LPush todolist writeblogs code enjoy makeprogress learn
done write cmd: [66, 56, 10, 8, 116, 111, 100, 111, 108, 105, 115, 116, 18, 10, 119, 114, 105, 116, 101, 98, 108, 111, 103, 115, 18, 4, 99, 111, 100, 101, 18, 5, 101, 110, 106, 111, 121, 18, 12, 109, 97, 107, 101, 112, 114, 111, 103, 114, 101, 115, 115, 18, 5, 108, 101, 97, 114, 110]
5

>>>LRange 0 100
parse cmd err: Invalid cmd parameter: not enough parameter for LRange
>>>LRange todolist 0 10
done write cmd: [82, 12, 10, 8, 116, 111, 100, 111, 108, 105, 115, 116, 24, 10]
writeblogs
code
enjoy
makeprogress
learn

>>>
```

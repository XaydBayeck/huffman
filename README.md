# huffman

[哈夫曼编码](https://zh.wikipedia.org/wiki/%E9%9C%8D%E5%A4%AB%E6%9B%BC%E7%BC%96%E7%A0%81)
是一种变长无损压缩编码方式，广泛应用于各种数据压缩场景中。
以单个字母为符号进行压缩时，哈夫曼编码速度略低，但结合字典编码等方式
可以发挥出十分优异的性能。

## 原理

```
          Symbols
             |
             |
            \|/
      {A B C D E F G H} 17 <---------- Weight
      0_______|__________1
      |                  |
     A 8         {B C D E F G H} 9
            0____________|_______________1
            |                            |
       {B C D} 5                     {E F G H} 4
      0_____|____1             0_________|_________1
      |          |             |                   |
     B 3      {C D} 2       {E F} 2             {G H} 2
          0______|___1      0__|___1          0____|____1
          |          |      |      |          |         |
         C 1        D 1    E 1    F 1        G 1       H 1

```

`Weight`是树节点中符号表中所有符号出现的总权重。根据符号出现在文本中的权重，将
出现概率低的符号归并到更深的子树中，构成哈夫曼编码树。

编解码时，`0`代表左枝，`1`代表右枝。比如:
- `A`的编码为`0`
- `B`的编码为`100`

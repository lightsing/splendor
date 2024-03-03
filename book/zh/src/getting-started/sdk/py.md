# Python

## 开发环境

Python 3.11

## 安装

```bash
// todo 发布到 pypi
```

## 实现

你需要继承 `PlayerActor` 类，并实现以下方法：

```python
class PlayerActor(ABC):
    @abstractmethod
    def get_action(self, snapshot: GameSnapshot) -> PlayerAction: ...

    @abstractmethod
    def drop_tokens(self, snapshot: GameSnapshot) -> DropTokensAction: ...

    @abstractmethod
    def select_noble(self, snapshot: GameSnapshot) -> SelectNoblesAction: ...
```

### get_action

当你的回合到来时，你需要返回一个 `PlayerAction`，它可能是：
- `TakeTokenAction`: 表示拿取 token 的行动，可以通过 `TakeTokenAction.three_different`（拿取至多三个不同种类的 token）或者 `TakeTokenAction.two_same`（拿取两个同种类的 token）创建
- `ReserveCardAction`: 表示保留卡牌的行动，可以通过 `ReserveCardAction.from_revealed`（从已经翻开的牌中保留卡牌）或者 `ReserveCardAction.from_pool`（从牌库保留卡牌）创建
- `BuyCardAction`: 表示购买卡牌的行动，可以通过 `BuyCardAction.from_revealed`（从已经翻开的牌中购买卡牌）或者 `BuyCardAction.from_reserved`（从保留的卡牌中购买卡牌）创建
- `NopAction`: 表示不做任何行动

### DropTokens

当你的回合结束，但是你拥有超过 10 个 token 时，你需要返回一个 `DropTokensAction`，表示你要丢弃的 token，通过 `DropTokensAction(tokens)` 创建。

### SelectNoble

当你的回合结束，并且你达到了多个贵族的要求时，你需要返回一个 `SelectNoblesAction`，表示你要选择的贵族的索引，通过 `SelectNoblesAction(idx)` 创建。

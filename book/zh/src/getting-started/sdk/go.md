# Golang

## 开发环境

Go1.22

## 安装

```bash
go get github.com/lightsing/splendor/sdk/go
```

## 实现

你需要实现 `PlayerActor` 接口。

```go
type PlayerActor interface {
	GetAction(*types.GameSnapshot) types.PlayerAction
	DropTokens(*types.GameSnapshot) types.ColorVec
	SelectNoble(*types.GameSnapshot) uint8
}
```

### GetAction

当你的回合到来时，你需要返回一个 `PlayerAction`，它可能是：
- `TakeTokenAction`: 表示拿取 token 的行动，可以通过 `NewTakeThreeDifferentTokenAction`（拿取至多三个不同种类的 token）或者 `NewTakeTwoSameTokenAction`（拿取两个同种类的 token）创建
- `ReserveCardAction`: 表示保留卡牌的行动，可以通过 `NewReserveCardFromRevealedAction`（从已经翻开的牌中保留卡牌）或者 `NewReserveCardFromPoolAction`（从牌库保留卡牌）创建
- `BuyCardAction`: 表示购买卡牌的行动，可以通过 `NewBuyCardFromRevealedAction`（从已经翻开的牌中购买卡牌）或者 `NewBuyCardFromReservedAction`（从保留的卡牌中购买卡牌）创建
- `NopAction`: 表示不做任何行动

### DropTokens

当你的回合结束，但是你拥有超过 10 个 token 时，你需要返回一个 `ColorVec`，表示你要丢弃的 token。

### SelectNoble

当你的回合结束，并且你达到了多个贵族的要求时，你需要返回一个 `uint8`，表示你要选择的贵族的索引。
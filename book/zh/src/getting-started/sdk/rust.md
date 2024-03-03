# Rust

## 开发环境

MSRV (Minimum Supported Rust Version): 1.76

## 安装

```bash
// todo 发布到 cargo
```

## 实现

你需要实现以下 trait:
```Rust
/// A player actor trait.
#[async_trait::async_trait]
pub trait PlayerActor: Send + Sync + Debug {
    /// It's the player's turn. Get the action to take.
    async fn get_action(&mut self, snapshot: GameSnapshot) -> Result<PlayerAction, ActorError>;

    /// The player has more than 10 tokens. Get the tokens to drop.
    async fn drop_tokens(&mut self, snapshot: GameSnapshot)
        -> Result<DropTokensAction, ActorError>;

    /// The player has more than 1 noble to visit. Select the noble to visit.
    async fn select_noble(
        &mut self,
        snapshot: GameSnapshot,
    ) -> Result<SelectNoblesAction, ActorError>;
}
```

### get_action

当你的回合到来时，你需要返回一个 `PlayerAction`，它可能是：
- `PlayerAction::TakeTokenAction`: 表示拿取 token 的行动，可以通过 `TakeTokenAction::ThreeDifferent`（拿取至多三个不同种类的 token）或者 `TakeTokenAction::TwoSame`（拿取两个同种类的 token）创建
- `PlayerAction::ReserveCardAction`: 表示保留卡牌的行动，当 `idx` 为 `None` 时，表示从牌库保留卡牌，否则表示从已经翻开的牌中保留卡牌
- `PlayerAction::BuyCardAction`: 表示购买卡牌的行动，由 `BuyCardSource` 表示购买来源，`uses` 表示购买的卡牌的话费
- `PlayerAction::NopAction`: 表示不做任何行动

### drop_tokens

当你的回合结束，但是你拥有超过 10 个 token 时，你需要返回一个 `DropTokensAction`，表示你要丢弃的 token。

### select_noble

当你的回合结束，并且你达到了多个贵族的要求时，你需要返回一个 `SelectNoblesAction`，表示你要选择的贵族的索引。

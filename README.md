# robotcyb-core

> Контракты рабочих задач, дедлайны, Modbus и энергетическая политика.

[![CI](https://github.com/c1cad4/robotcyb-core/actions/workflows/ci.yml/badge.svg)](https://github.com/c1cad4/robotcyb-core/actions/workflows/ci.yml)

[Карта экосистемы](https://github.com/c1cad4/cybOS) · [Архитектура](docs/ECOSYSTEM.md) · [Интеграция в cybOS](https://github.com/c1cad4/CybOS-demo)

## Назначение

Контракты рабочих задач, дедлайны, Modbus и энергетическая политика. Этот репозиторий владеет своей областью; приложение и его UI остаются в `CybOS-demo`.

## Начать работу

Репозитории размещаются рядом в одной рабочей папке. Для полного окружения используйте `cybOS/tools/bootstrap.py` и `cybLaunch/launcher.py`; точные ревизии публикуются в lock-файлах интегратора.

```bash
cd robotcyb-core
cargo test --locked -j 4
cargo run --locked --example demo
```

## Структура

- `src/` — публичная библиотека и её тесты.
- `Cargo.lock` — воспроизводимые зависимости.
- `examples/` — небольшой проверяемый пример; данные с меткой demo являются фикстурами.
- `docs/ECOSYSTEM.md` — границы компонента и связи.

## Визия

[Исходная идея и план проекта](docs/VISION.md).

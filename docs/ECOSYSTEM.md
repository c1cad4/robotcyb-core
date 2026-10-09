# robotcyb-core в экосистеме cybOS

Контракты рабочих задач, дедлайны, Modbus и энергетическая политика.

[Общая карта](https://github.com/c1cad4/cybOS) · [Тестовый стенд](https://github.com/c1cad4/CybOS-demo) · [Каталог компонентов](https://github.com/c1cad4/cybOS/blob/main/ecosystem.json)

## Ответственность

Категория: `intelligence`. Тип компонента: `library`.

## Проверка

Из корня репозитория:

```bash
cargo test --locked -j 4
```

Код выделен из проверенного CybOS-demo. Источник и контрольные суммы исходных файлов записаны в `PROVENANCE.json`; дальнейшие изменения поддерживаются здесь. В cybOS используется эта библиотека через Cargo path dependency.

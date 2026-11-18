> Перевод выполнен ИИ, носителями языка пока не проверен. Приоритет имеет японский оригинал (корень репозитория).

# pl2pl
**Конвертер между языками программирования, фреймворками и операционными системами** (v0.1.0 опубликована, в разработке). Есть установщики для Windows (x64/ARM64), Linux (amd64/arm64) и macOS (arm64/x64). Конвертация выполняется в установленном приложении для ПК (CLI); веб-версия содержит только руководства и загрузки.
Конвертация осторожная: **то, что нельзя преобразовать, никогда не отбрасывается молча; оно остаётся как `TODO` и указывается в отчёте с причиной.**

## Что работает сейчас
| Команда | Описание |
|---|---|
| `pl2pl cobol` | COBOL (IBM) + JCL → GnuCOBOL (Linux) + bash. Проверено реальной сборкой и запуском в GnuCOBOL. |
| `pl2pl omp` | Добавляет `#pragma omp parallel for` к независимым циклам C/C++ (с редукцией сумм). Результат совпадает с оригиналом. |
| `pl2pl php` | Laravel / Symfony → FastAPI (см. ниже). Сквозное тестирование на настоящих FastAPI, SQLite и Jinja2. |
| `pl2pl web` | Определяет Java / COBOL / Flask·Django и передаёт java_to, cobol_to, 2fastapi, 2poem. Проверено на реальных файлах. |
| `pl2pl cport` | C для UNIX → заголовок совместимости + механическая замена + рекомендации по переносу. Проверено с gcc. |
| `pl2pl pyhpc` | Numba `@njit` для числовых функций Python; `--mpi` создаёт заготовку mpi4py. Тот же результат под Numba. |
| `pl2pl kokkos` | Независимые циклы → Kokkos (`parallel_for` / `parallel_reduce`). Собрано и запущено с Kokkos 4.5.01 (Serial). |
| `pl2pl gpu` | (Прототип) циклы → `.cu` в стиле CUDA / HLSL. Только вывод; выполнение на open-cuda / open-directx не проверено. |
| `pl2pl llm setup` / `llm start` | Запускает встроенный aruaru-llm и устанавливает модель Qwen2.5 по объёму свободной памяти. Проверены только логика выбора и --dry-run. |
| `pl2pl doctor` | Сведения о процессоре через open-cpu (встроен). |

## Конвертация PHP (`pl2pl php`)
- **Маршруты**: Laravel `Route::get/post/put/patch/delete/any/resource/apiResource/view`, группы `prefix`, Symfony `#[Route]`.
- **Тела функций**: простой `return`, несколько операторов, `response()->json($x, 201)`, `noContent()`.
- **Eloquent → SQLAlchemy** (`db.py`, `models.py`): столбцы берутся из миграций или `$fillable`, остальные поля — в JSON-столбец `extra`; БД задаётся через `DATABASE_URL` (по умолчанию SQLite).
- **Query builder**: `where / orWhere / whereIn / orderBy / latest / oldest / limit / take / offset / skip`, `paginate / simplePaginate` (`?page=N`, JSON как в Laravel, `links()` в Blade), `get / first / firstOrFail / count / exists / delete`, `$request->query('k', значение)`.
- **Связи**: `hasMany / hasOne / belongsTo / belongsToMany` (промежуточная таблица создаётся), `with`, `whereHas / has / whereDoesntHave / doesntHave`, `attach / detach / sync`.
- **Blade → Jinja2**: вывод, `@if`, `@foreach`, `@forelse`, `@for`, `@extends/@section/@yield/@include` и др.
- **Middleware → `Depends`**: `auth` (простая проверка Bearer, **заменить настоящей аутентификацией**), `guest`, `throttle:N,M`, собственный middleware «отклонить по условию, иначе `return $next($request)`».

## Планируется (ещё не сделано)
- PHP: вложенный `with('a.b')`, связи `morph`, `cursorPaginate`, компоненты/стеки/директивы авторизации Blade, middleware, обрабатывающий ответ после `$next`.
- Включить COBOL_to и Java_to в pl2pl, когда они созреют, и удалить исходные репозитории (указание владельца; удаление необратимо, поэтому после включения будет повторное подтверждение).
- Общее промежуточное представление (серверная часть, БД/SQL, интерфейс), чтобы для N источников и M целей требовалось N+M компонентов, а не N×M.
- Интеграция с aruaru-llm (пояснения и перевод естественного языка отделены от конвертеров; конвертеры никогда не зависят от LLM).
- Выполнение Kokkos на GPU и `Kokkos::View`; запуск ядер на open-cuda / open-directx.
- Запуск новых примеров PHP в CI на Linux arm64 и macOS; новые пары языков из `docs/LANGUAGES.md` (все комбинации строить не будем).

## Примечания
Решения осторожные: при возможной зависимости или побочном эффекте конструкция **остаётся неконвертированной, причина указывается в отчёте**; производительность не гарантируется. Проверялись только небольшие самодельные примеры, а не крупные реальные программы.
Документы: `docs/DESIGN.md`, `docs/LANGUAGES.md`, `docs/COMBINATIONS.md`.

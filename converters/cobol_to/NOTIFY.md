# 自動更新しきれない場合の通知 / Notification when auto-update is not enough / Benachrichtigung bei nicht automatisierbaren Updates / Уведомление, если автообновление не справилось

## 日本語
COBOL_to の依存(Rust・Poem/RPoem・Python・FastAPI/MFastAPI・Next.js など)は、`version-control` が毎晩走査して最新を確認します。
マイナー/パッチ更新は自動保守(`aruaru-llm`・`open-cuda`・`open-cpu`・`open-directx`)に任せます。
**メジャー更新、セキュリティ修正の取り込み失敗、クローンの遅れ・分岐など、自動で済まないもの**は、あらかじめ登録した**2 つのメールアドレス**へ「Claude Code に貼り付けるメッセージ」として送り、Claude に更新対応してもらいます。

- 登録場所: `version-control/config.toml` の `[notify] email_to`(下の `notify.example.toml` を参照)。2 件とも利用者が自分で変更できます。
- 同じ内容は 1 度だけ送ります。内容が変わると再送します。
- `version-control` は依存を書き換えず `git pull` もしません。更新は受け取った Claude が行い、テストが通ってから反映します。
- メールアドレスはリポジトリに書きません(`config.toml` は公開しない)。
- 走査の起点(クローンを置いたドライブ/フォルダ `root`、GitHub の組織/ユーザー名 `owner`)は**各ユーザーが自分で指定**します。`F:/` や `aon-co-jp` は作者の例です。

## English
`version-control` scans COBOL_to's dependencies (Rust, Poem/RPoem, Python, FastAPI/MFastAPI, Next.js, ...) every night. Minor/patch updates are left to the automatic maintainers (`aruaru-llm`, `open-cuda`, `open-cpu`, `open-directx`).
**Anything that cannot be finished automatically** (major updates, failed security fixes, a clone that is behind or has diverged) is sent as a paste-ready Claude Code message to **two pre-registered e-mail addresses**, so Claude can carry out the update.

- Where: `[notify] email_to` in `version-control/config.toml` (see `notify.example.toml`). The user can change both addresses.
- Each message is sent once; it is re-sent only when its content changes.
- `version-control` never rewrites dependencies or runs `git pull`; Claude applies the update and lands it only after the tests pass.
- E-mail addresses are never committed (`config.toml` stays private).
- The scan start points (`root`, the drive/folder holding your clones, and `owner`, your GitHub organisation/user) are **specified by each user**. `F:/` and `aon-co-jp` are the author's examples.

## Deutsch (KI-Übersetzung; noch nicht von Muttersprachlern geprüft)
`version-control` prüft jede Nacht die Abhängigkeiten von COBOL_to (Rust, Poem/RPoem, Python, FastAPI/MFastAPI, Next.js usw.). Minor- und Patch-Updates übernehmen die automatischen Wartungsdienste (`aruaru-llm`, `open-cuda`, `open-cpu`, `open-directx`).
**Alles, was sich nicht automatisch erledigen lässt** (Major-Updates, gescheiterte Sicherheitskorrekturen, ein veralteter oder abgewichener Klon), wird als einfügbare Claude-Code-Nachricht an **zwei vorab hinterlegte E-Mail-Adressen** geschickt, damit Claude das Update durchführt.

- Ort: `[notify] email_to` in `version-control/config.toml` (siehe `notify.example.toml`). Beide Adressen kann der Benutzer selbst ändern.
- Jede Nachricht wird nur einmal gesendet und erneut, wenn sich der Inhalt ändert.
- `version-control` ändert keine Abhängigkeiten und führt kein `git pull` aus; Claude spielt das Update ein und übernimmt es erst, wenn die Tests bestehen.
- E-Mail-Adressen werden nie eingecheckt (`config.toml` bleibt privat).
- Die Startpunkte des Scans (`root`: Laufwerk/Verzeichnis mit Ihren Klonen, `owner`: Ihre GitHub-Organisation bzw. Ihr Benutzername) **legt jeder Benutzer selbst fest**. `F:/` und `aon-co-jp` sind nur Beispiele des Autors.

## Русский (перевод выполнен ИИ; носителями языка пока не проверен)
`version-control` каждую ночь проверяет зависимости COBOL_to (Rust, Poem/RPoem, Python, FastAPI/MFastAPI, Next.js и др.). Минорные и патч-обновления берут на себя автоматические службы (`aruaru-llm`, `open-cuda`, `open-cpu`, `open-directx`).
**Всё, что нельзя выполнить автоматически** (мажорные обновления, неудачные исправления безопасности, отставший или разошедшийся клон), отправляется в виде сообщения для вставки в Claude Code на **два заранее зарегистрированных адреса электронной почты**, чтобы Claude выполнил обновление.

- Где: `[notify] email_to` в `version-control/config.toml` (см. `notify.example.toml`). Оба адреса пользователь может изменить сам.
- Каждое сообщение отправляется один раз; повторно — только при изменении содержимого.
- `version-control` не изменяет зависимости и не выполняет `git pull`; Claude применяет обновление и вносит его только после успешных тестов.
- Адреса электронной почты в репозиторий не попадают (`config.toml` остаётся закрытым).
- Начальные точки проверки (`root` — диск или папка с вашими клонами, `owner` — ваша организация или пользователь GitHub) **каждый пользователь задаёт сам**. `F:/` и `aon-co-jp` — лишь примеры автора.

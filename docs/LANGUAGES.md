# 変換元候補の一覧 / Source candidates (2026-11-18)
知識に基づく一覧(検索で最新性は未検証)。向き = サーバー側ルート中心の変換に向くか(◎ 向く / ○ 可 / △ 難 / ✕ 不向き)。
*Compiled from knowledge, not freshly searched. Fit = suitability for route-centred server conversion.*

## 0. 利用者が想定した変換元 / Sources the owner proposed
HTML5, CSS3, JavaScript, TypeScript, jQuery, WunderGraph Cosmo, React, Next.js, Nuxt.js, Bootstrap, Access, Access VBA, VB, VB.net, C, C++, VC++, C#.net, F#, F#.net, COBOL, PHP, Python, Java, Ruby, MySQL, MariaDB, Oracle, Snowflake, CockroachDB, aruaru-db.
注 / Note: WunderGraph Cosmo は言語ではなくルーター(変換元というより接続先の可能性、要確認)。Access は DB・画面・VBA が一体。/ Cosmo is a router, not a language; Access mixes DB, UI and VBA.

## A. 既出言語のフレームワーク / Frameworks of the named languages
| 言語 Language | フレームワーク Frameworks | 向き Fit |
|---|---|---|
| PHP | Laravel, Symfony, CakePHP, CodeIgniter, Yii, Slim, Lumen, Laminas, FuelPHP, WordPress plugins | ◎ |
| Ruby | Rails, Sinatra, Hanami, Padrino, Grape, Roda | ◎ |
| C# / VB.NET | ASP.NET Core (MVC/Minimal API/Web API), ASP.NET Web Forms, Nancy, ServiceStack | ◎ (Web Forms = UI) |
| Python | Flask, Django(DRF), FastAPI, Tornado, Pyramid, Bottle, Falcon, Sanic, aiohttp, Starlette, Litestar | ◎ (Flask/Django/FastAPI done by 2FastAPI etc.) |
| JS/TS server | Express, Koa, Fastify, NestJS, Hapi, AdonisJS, Sails, LoopBack, tRPC, Hono, Elysia | ◎ |
| C/C++ | Drogon, Crow, Pistache, Oat++, CppCMS, Wt, Qt(UI) | ○ |
| Java | Spring, JAX-RS, Servlet, Spark, Javalin… (Java_to, 58 entries) | ◎ done |
| COBOL | COBOL_to | done (minimal) |

## B. 他の主要言語 / Other major languages
| 言語 | フレームワーク | 向き |
|---|---|---|
| Go | Gin, Echo, Fiber, Chi, Beego, Buffalo, gorilla/mux | ◎ |
| Rust | Actix-web, Axum, Rocket, Warp, Poem (target), Loco | △ (same family as targets) |
| Kotlin | Ktor, Spring, Micronaut, http4k, Javalin | ◎ |
| Scala | Play, Akka HTTP, http4s, Finatra, Scalatra, ZIO HTTP | ○ |
| Groovy | Grails, Ratpack, Micronaut | ○ |
| Swift | Vapor, Hummingbird | ○ |
| Dart | Shelf, Dart Frog, Serverpod | ○ |
| Elixir | Phoenix, Plug, Ash | ○ |
| Erlang | Cowboy, Nitrogen | △ |
| Clojure | Ring, Compojure, Pedestal, Luminus | △ |
| Haskell | Yesod, Servant, Scotty, IHP | △ |
| OCaml | Dream, Opium | △ |
| Perl | Mojolicious, Catalyst, Dancer2 | ○ |
| Lua | OpenResty, Lapis | △ |
| R / Julia | Plumber, Shiny / Genie.jl | △ |
| Zig / Nim / Crystal | Zap / Jester / Kemal, Lucky | △ |
| Delphi / Pascal | VCL(UI), Horse | △ |
| Ada | AWS | △ |
| Fortran | (no mainstream web framework) | ✕ |

## C. レガシー・業務系 / Legacy and business
RPG (IBM i) ○, PL/I ○, JCL △ (batch definitions), ABAP (SAP) △, Natural/Adabas △, PowerBuilder △, Visual FoxPro △, Smalltalk △, Excel VBA / Google Apps Script △.

## D. DB / SQL(別のIRが必要 / needs its own IR)
PostgreSQL, SQLite, SQL Server (T-SQL), DB2, Teradata, BigQuery, Redshift, PL/SQL, PL/pgSQL, MongoDB (MQL), Redis, Cassandra (CQL), DynamoDB, Neo4j (Cypher), Elasticsearch, ClickHouse, TiDB, Vitess, OceanBase (+ the owner's list: MySQL, MariaDB, Oracle, Snowflake, CockroachDB, aruaru-db).

## E. 画面側(別のIRが必要 / needs its own IR; target = RPoem Rust→WASM)
- UI frameworks: Angular, Vue, Svelte, SvelteKit, Solid, Qwik, Astro, Remix, Gatsby, Preact, Ember, Backbone, AngularJS (+ owner's list: React, Next.js, Nuxt.js, jQuery).
- Styling: Tailwind, Sass/Less, Bulma, Foundation, Material UI (+ Bootstrap, CSS3).
- Obsolete: Flash/ActionScript, Silverlight, Java Applet, ActiveX.
- Mobile/desktop: React Native, Flutter, Xamarin/.NET MAUI, Ionic/Cordova, Electron, Tauri, WPF, WinForms, Swing, Android (Kotlin/Java), iOS (Swift/Objective-C).
- Templates: Jinja2, Blade, ERB, Twig, Handlebars, Thymeleaf, Razor.

## F. その他 / Other
GraphQL (Apollo, Hasura), gRPC/protobuf, SOAP/WSDL, OpenAPI (spec → code), Terraform/Ansible, shell, PowerShell, Mojo (RMojo is a sibling).

## 最初の候補 / Candidate for the first one (suggestion, undecided)
PHP (same four targets as existing tools, plenty of real projects, legacy demand in Japan, a third language to extract a shared IR). Runners-up: Go or Kotlin (route-centred); SQL dialects → aruaru-db as a separate line.

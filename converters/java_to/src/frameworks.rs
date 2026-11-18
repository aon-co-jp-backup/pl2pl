//! Java のフレームワーク/ライブラリ台帳(2000年頃〜2026-11)と自動判定。
//! 判定の根拠: import の接頭辞、pom.xml / build.gradle の依存名、設定ファイル(web.xml・struts.xml・faces-config.xml・conf/routes 等)。
//! 年代は各プロジェクトの公開年の目安(出典と未確認事項は docs/RESEARCH.md)。

use crate::parse::Unit;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Support {
    /// ルート・パラメータ・本体を変換する
    Convert,
    /// 検出して報告し、移行先の対応を示す(変換はしない)
    Report,
}

pub struct Fw {
    pub name: &'static str,
    pub since: u16,
    pub kind: &'static str,
    /// import 接頭辞・ビルド/設定ファイル内の文字列
    pub markers: &'static [&'static str],
    pub python: &'static str,
    pub rust: &'static str,
    pub support: Support,
}

macro_rules! fw {
    ($n:expr, $y:expr, $k:expr, [$($m:expr),*], $py:expr, $rs:expr, $s:ident) => {
        Fw { name: $n, since: $y, kind: $k, markers: &[$($m),*], python: $py, rust: $rs, support: Support::$s }
    };
}

pub const CATALOG: &[Fw] = &[
    // --- Servlet / JSP 世代 ---
    fw!("Servlet/JSP (javax)", 1999, "web", ["javax.servlet", "javax.servlet.http"], "FastAPI のルート", "Poem の #[handler]", Convert),
    fw!("Servlet/JSP (jakarta)", 2019, "web", ["jakarta.servlet"], "FastAPI のルート", "Poem の #[handler]", Convert),
    fw!("JSTL/EL", 2002, "view", ["javax.servlet.jsp", "jakarta.servlet.jsp", "taglibs.standard"], "Jinja2 テンプレート", "open-runo-view(RPoem)", Report),
    fw!("Struts 1", 2000, "web", ["org.apache.struts.action", "org.apache.struts.actions", "struts-core", "struts-config.xml"], "FastAPI のルート(Action→関数)", "Poem の #[handler]", Report),
    fw!("Struts 2", 2006, "web", ["org.apache.struts2", "com.opensymphony.xwork2", "struts.xml", "struts2-core"], "FastAPI のルート", "Poem の #[handler]", Report),
    fw!("WebWork", 2002, "web", ["com.opensymphony.webwork"], "FastAPI のルート", "Poem の #[handler]", Report),
    fw!("Spring Framework / MVC", 2003, "web", ["org.springframework.web", "org.springframework.stereotype", "org.springframework.beans"], "FastAPI のルート+Depends", "Poem のルート+共有状態", Convert),
    fw!("Spring Boot", 2014, "web", ["org.springframework.boot", "spring-boot-starter"], "FastAPI アプリ", "Poem アプリ", Convert),
    fw!("Spring WebFlux", 2017, "web", ["org.springframework.web.reactive", "reactor.core", "spring-boot-starter-webflux"], "FastAPI(async)", "Poem(async)", Convert),
    fw!("Spring Data / JPA", 2008, "data", ["org.springframework.data", "javax.persistence", "jakarta.persistence"], "SQLAlchemy / SQLModel", "sqlx / SeaORM", Report),
    fw!("Spring Security", 2004, "security", ["org.springframework.security", "spring-boot-starter-security"], "FastAPI の Depends + python-jose", "Poem のミドルウェア + jsonwebtoken", Report),
    fw!("Spring Cloud / Feign / Hystrix", 2015, "cloud", ["org.springframework.cloud", "feign.", "com.netflix.hystrix", "io.github.resilience4j"], "httpx + tenacity", "reqwest + tower", Report),
    fw!("Spring Batch / Integration", 2007, "batch", ["org.springframework.batch", "org.springframework.integration"], "スクリプト+スケジューラ", "tokio タスク", Report),
    fw!("Spring Roo", 2009, "web", ["org.springframework.roo"], "FastAPI アプリ", "Poem アプリ", Report),
    fw!("JSF (javax/jakarta)", 2004, "web", ["javax.faces", "jakarta.faces", "org.primefaces", "org.richfaces", "faces-config.xml"], "FastAPI+Jinja2", "RPoem の open-runo-view", Report),
    fw!("Apache Tapestry", 2000, "web", ["org.apache.tapestry", "org.apache.tapestry5"], "FastAPI+Jinja2", "RPoem の open-runo-view", Report),
    fw!("Apache Wicket", 2005, "web", ["org.apache.wicket"], "FastAPI+Jinja2", "RPoem の open-runo-view", Report),
    fw!("Stripes", 2005, "web", ["net.sourceforge.stripes"], "FastAPI のルート", "Poem の #[handler]", Report),
    fw!("Seasar2 / SAStruts / S2Dao", 2004, "web", ["org.seasar.", "org.seasar.struts", "org.seasar.dao"], "FastAPI のルート", "Poem の #[handler]", Report),
    fw!("GWT", 2006, "web", ["com.google.gwt"], "FastAPI + TypeScript", "RPoem(Rust→WASM)", Report),
    fw!("Vaadin", 2009, "web", ["com.vaadin"], "FastAPI + フロント", "RPoem の open-runo-view", Report),
    fw!("ZK", 2005, "web", ["org.zkoss"], "FastAPI + フロント", "RPoem の open-runo-view", Report),
    fw!("Play Framework (Java)", 2007, "web", ["play.mvc", "play.libs", "play.api", "com.typesafe.play", "conf/routes"], "FastAPI のルート", "Poem の #[handler]", Report),
    fw!("Grails / Groovy", 2006, "web", ["grails.", "org.grails"], "FastAPI(Groovy は対象外)", "Poem", Report),
    fw!("Vert.x", 2011, "web", ["io.vertx"], "FastAPI(async)", "Poem(async)", Convert),
    fw!("Spark Java", 2011, "web", ["spark.Spark", "com.sparkjava"], "FastAPI のルート", "Poem の #[handler]", Convert),
    fw!("Javalin", 2017, "web", ["io.javalin"], "FastAPI のルート", "Poem の #[handler]", Convert),
    fw!("Dropwizard", 2011, "web", ["io.dropwizard", "com.yammer.dropwizard", "com.codahale.metrics"], "FastAPI アプリ", "Poem アプリ", Convert),
    fw!("JAX-RS (javax)", 2008, "web", ["javax.ws.rs"], "FastAPI のルート", "Poem の #[handler]", Convert),
    fw!("JAX-RS (jakarta)", 2019, "web", ["jakarta.ws.rs"], "FastAPI のルート", "Poem の #[handler]", Convert),
    fw!("Jersey", 2008, "web", ["org.glassfish.jersey", "com.sun.jersey"], "FastAPI のルート", "Poem の #[handler]", Convert),
    fw!("RESTEasy", 2008, "web", ["org.jboss.resteasy"], "FastAPI のルート", "Poem の #[handler]", Convert),
    fw!("Apache CXF", 2006, "web", ["org.apache.cxf"], "FastAPI のルート(SOAP は zeep 等)", "Poem(SOAP は対象外)", Report),
    fw!("Restlet", 2007, "web", ["org.restlet"], "FastAPI のルート", "Poem の #[handler]", Report),
    fw!("JAX-WS / SOAP / Axis", 2000, "soap", ["javax.jws", "javax.xml.ws", "jakarta.jws", "jakarta.xml.ws", "org.apache.axis"], "zeep / spyne(別途)", "対象外(要手作業)", Report),
    fw!("Micronaut", 2018, "web", ["io.micronaut"], "FastAPI のルート+Depends", "Poem のルート", Convert),
    fw!("Quarkus", 2019, "web", ["io.quarkus", "io.smallrye", "quarkus-resteasy", "quarkus-rest"], "FastAPI のルート", "Poem の #[handler]", Convert),
    fw!("Helidon", 2018, "web", ["io.helidon"], "FastAPI のルート", "Poem の #[handler]", Report),
    fw!("MicroProfile", 2016, "web", ["org.eclipse.microprofile", "org.eclipse.microprofile.rest.client"], "FastAPI + 設定", "Poem + 設定", Report),
    fw!("Jakarta EE / Java EE (EJB・CDI)", 1999, "enterprise", ["javax.ejb", "jakarta.ejb", "javax.inject", "jakarta.inject", "javax.enterprise", "jakarta.enterprise"], "Depends / 単純なクラス", "構造体+共有状態", Report),
    fw!("Ratpack", 2014, "web", ["ratpack."], "FastAPI のルート", "Poem の #[handler]", Report),
    fw!("Jooby", 2014, "web", ["io.jooby", "org.jooby"], "FastAPI のルート", "Poem の #[handler]", Report),
    fw!("Armeria", 2015, "web", ["com.linecorp.armeria"], "FastAPI のルート", "Poem の #[handler]", Report),
    fw!("Netty / Undertow / Jetty / Tomcat 埋め込み", 2004, "server", ["io.netty", "io.undertow", "org.eclipse.jetty", "org.mortbay.jetty", "org.apache.catalina"], "uvicorn / hypercorn", "hyper / Poem サーバー", Report),
    fw!("gRPC Java", 2015, "rpc", ["io.grpc"], "grpcio / Connect", "tonic / Connect", Report),
    fw!("Apache Camel", 2007, "integration", ["org.apache.camel"], "個別実装", "個別実装", Report),
    fw!("Dubbo", 2008, "rpc", ["org.apache.dubbo", "com.alibaba.dubbo"], "gRPC / Connect", "gRPC / Connect", Report),
    // --- 永続化・補助 ---
    fw!("JDBC", 1997, "data", ["java.sql", "javax.sql"], "sqlite3 / psycopg", "sqlx", Report),
    fw!("Hibernate", 2001, "data", ["org.hibernate"], "SQLAlchemy", "SeaORM / sqlx", Report),
    fw!("MyBatis / iBATIS", 2002, "data", ["org.apache.ibatis", "org.mybatis"], "SQLAlchemy / 生SQL", "sqlx", Report),
    fw!("jOOQ", 2009, "data", ["org.jooq"], "SQLAlchemy Core", "sqlx / SeaQuery", Report),
    fw!("Lombok", 2009, "util", ["lombok"], "dataclass / pydantic", "derive マクロ", Convert),
    fw!("Jackson / Gson", 2004, "util", ["com.fasterxml.jackson", "org.codehaus.jackson", "com.google.gson", "tools.jackson"], "pydantic / json", "serde", Convert),
    fw!("Bean Validation", 2009, "util", ["javax.validation", "jakarta.validation"], "pydantic Field", "validator クレート", Report),
    fw!("Thymeleaf / FreeMarker / Velocity", 2003, "view", ["org.thymeleaf", "freemarker.", "org.apache.velocity"], "Jinja2", "open-runo-view(RPoem)", Report),
    fw!("Kafka / RabbitMQ / JMS", 2000, "messaging", ["org.apache.kafka", "com.rabbitmq", "javax.jms", "jakarta.jms"], "aiokafka / pika", "rdkafka / lapin", Report),
    fw!("Swing / AWT / JavaFX(画面アプリ)", 1997, "gui", ["javax.swing", "java.awt", "javafx."], "対象外(別途 GUI を選ぶ)", "対象外", Report),
    fw!("Android(Java)", 2008, "mobile", ["android.", "androidx."], "対象外", "対象外", Report),
];

#[derive(Clone, Debug)]
pub struct Found {
    pub name: &'static str,
    pub since: u16,
    pub kind: &'static str,
    pub python: &'static str,
    pub rust: &'static str,
    pub support: Support,
    pub evidence: Vec<String>,
}

/// `units`(解析済み Java)と `extra`(ファイル名, 内容: pom.xml・build.gradle・各種 XML・conf/routes など)から判定する。
pub fn detect(units: &[Unit], extra: &[(String, String)]) -> Vec<Found> {
    let mut out = vec![];
    for f in CATALOG {
        let mut ev: Vec<String> = vec![];
        for m in f.markers {
            // import は `javax.servlet` のような接頭辞で、ドット区切りの境界でそろえる
            'u: for u in units {
                for im in &u.imports {
                    let hit = im == m || im.starts_with(&format!("{m}.")) || (m.ends_with('.') && im.starts_with(m));
                    if hit {
                        ev.push(format!("{}: import {}", short(&u.file), im));
                        break 'u;
                    }
                }
            }
            for (name, text) in extra {
                let n = name.to_lowercase();
                let is_build = n.ends_with("pom.xml") || n.ends_with(".gradle") || n.ends_with(".gradle.kts");
                let is_cfg = n.ends_with(".xml") || n.ends_with("routes") || n.ends_with(".conf") || n.ends_with(".properties");
                let needle = m.trim_end_matches('.');
                if (is_build && text.contains(&needle.replace('.', "-")))
                    || (is_build && text.contains(needle))
                    || (is_cfg && short(name).eq_ignore_ascii_case(needle))
                    || (is_cfg && !is_build && name_matches(&n, needle))
                {
                    ev.push(format!("{}: {}", short(name), needle));
                    break;
                }
            }
        }
        if !ev.is_empty() {
            ev.sort();
            ev.dedup();
            ev.truncate(4);
            out.push(Found { name: f.name, since: f.since, kind: f.kind, python: f.python, rust: f.rust, support: f.support, evidence: ev });
        }
    }
    // javax.servlet は jakarta.servlet とは別項目、JAX-RS(javax)が見つかれば Jersey 等と併記される
    out
}

fn short(p: &str) -> String {
    p.replace('\\', "/").rsplit('/').next().unwrap_or(p).to_string()
}

fn name_matches(file_lower: &str, needle: &str) -> bool {
    // `struts.xml` / `faces-config.xml` / `conf/routes` のような設定ファイル名の一致
    let nl = needle.to_lowercase();
    nl.contains('.') && !nl.contains('/') && file_lower.ends_with(&nl) || (nl == "conf/routes" && file_lower.replace('\\', "/").ends_with("conf/routes"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::parse_unit;

    #[test]
    fn detects_by_import_and_build_file() {
        let u = parse_unit("A.java", "import org.springframework.boot.SpringApplication; import jakarta.ws.rs.GET; class A {}");
        let extra = vec![("pom.xml".to_string(), "<artifactId>quarkus-resteasy</artifactId>".to_string()), ("struts.xml".to_string(), "<struts/>".to_string())];
        let f = detect(&[u], &extra);
        let names: Vec<_> = f.iter().map(|x| x.name).collect();
        assert!(names.contains(&"Spring Boot"));
        assert!(names.contains(&"JAX-RS (jakarta)"));
        assert!(names.contains(&"Quarkus"));
        assert!(names.contains(&"Struts 2"));
        assert!(!names.contains(&"Struts 1"));
    }
    #[test]
    fn catalog_covers_2000_to_2018_and_later() {
        assert!(CATALOG.len() >= 55);
        assert!(CATALOG.iter().any(|f| f.since <= 2000));
        assert!(CATALOG.iter().any(|f| f.since >= 2019));
    }
}

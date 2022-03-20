(function() {var implementors = {};
implementors["axum"] = [{"text":"impl&lt;T, B&gt; <a class=\"trait\" href=\"axum/extract/trait.FromRequest.html\" title=\"trait axum::extract::FromRequest\">FromRequest</a>&lt;B&gt; for <a class=\"struct\" href=\"axum/struct.Json.html\" title=\"struct axum::Json\">Json</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"serde/de/trait.DeserializeOwned.html\" title=\"trait serde::de::DeserializeOwned\">DeserializeOwned</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;B: <a class=\"trait\" href=\"http_body/trait.Body.html\" title=\"trait http_body::Body\">HttpBody</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;B::<a class=\"associatedtype\" href=\"http_body/trait.Body.html#associatedtype.Data\" title=\"type http_body::Body::Data\">Data</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;B::<a class=\"associatedtype\" href=\"http_body/trait.Body.html#associatedtype.Error\" title=\"type http_body::Body::Error\">Error</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"type\" href=\"axum/type.BoxError.html\" title=\"type axum::BoxError\">BoxError</a>&gt;,&nbsp;</span>","synthetic":false,"types":["axum::json::Json"]},{"text":"impl&lt;B, T&gt; <a class=\"trait\" href=\"axum/extract/trait.FromRequest.html\" title=\"trait axum::extract::FromRequest\">FromRequest</a>&lt;B&gt; for <a class=\"struct\" href=\"axum/extract/struct.ConnectInfo.html\" title=\"struct axum::extract::ConnectInfo\">ConnectInfo</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;B: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + 'static,&nbsp;</span>","synthetic":false,"types":["axum::extract::connect_info::ConnectInfo"]},{"text":"impl&lt;T, B&gt; <a class=\"trait\" href=\"axum/extract/trait.FromRequest.html\" title=\"trait axum::extract::FromRequest\">FromRequest</a>&lt;B&gt; for <a class=\"struct\" href=\"axum/extract/struct.Path.html\" title=\"struct axum::extract::Path\">Path</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"serde/de/trait.DeserializeOwned.html\" title=\"trait serde::de::DeserializeOwned\">DeserializeOwned</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;B: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,&nbsp;</span>","synthetic":false,"types":["axum::extract::path::Path"]},{"text":"impl&lt;T, B, const N:&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/std/primitive.u64.html\">u64</a>&gt; <a class=\"trait\" href=\"axum/extract/trait.FromRequest.html\" title=\"trait axum::extract::FromRequest\">FromRequest</a>&lt;B&gt; for <a class=\"struct\" href=\"axum/extract/struct.ContentLengthLimit.html\" title=\"struct axum::extract::ContentLengthLimit\">ContentLengthLimit</a>&lt;T, N&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"axum/extract/trait.FromRequest.html\" title=\"trait axum::extract::FromRequest\">FromRequest</a>&lt;B&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;T::<a class=\"associatedtype\" href=\"axum/extract/trait.FromRequest.html#associatedtype.Rejection\" title=\"type axum::extract::FromRequest::Rejection\">Rejection</a>: <a class=\"trait\" href=\"axum/response/trait.IntoResponse.html\" title=\"trait axum::response::IntoResponse\">IntoResponse</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;B: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,&nbsp;</span>","synthetic":false,"types":["axum::extract::content_length_limit::ContentLengthLimit"]},{"text":"impl&lt;T, B&gt; <a class=\"trait\" href=\"axum/extract/trait.FromRequest.html\" title=\"trait axum::extract::FromRequest\">FromRequest</a>&lt;B&gt; for <a class=\"struct\" href=\"axum/extract/struct.Extension.html\" title=\"struct axum::extract::Extension\">Extension</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + 'static,<br>&nbsp;&nbsp;&nbsp;&nbsp;B: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,&nbsp;</span>","synthetic":false,"types":["axum::extract::extension::Extension"]},{"text":"impl&lt;T, B&gt; <a class=\"trait\" href=\"axum/extract/trait.FromRequest.html\" title=\"trait axum::extract::FromRequest\">FromRequest</a>&lt;B&gt; for <a class=\"struct\" href=\"axum/extract/struct.Form.html\" title=\"struct axum::extract::Form\">Form</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"serde/de/trait.DeserializeOwned.html\" title=\"trait serde::de::DeserializeOwned\">DeserializeOwned</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;B: <a class=\"trait\" href=\"http_body/trait.Body.html\" title=\"trait http_body::Body\">HttpBody</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;B::<a class=\"associatedtype\" href=\"http_body/trait.Body.html#associatedtype.Data\" title=\"type http_body::Body::Data\">Data</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;B::<a class=\"associatedtype\" href=\"http_body/trait.Body.html#associatedtype.Error\" title=\"type http_body::Body::Error\">Error</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"type\" href=\"axum/type.BoxError.html\" title=\"type axum::BoxError\">BoxError</a>&gt;,&nbsp;</span>","synthetic":false,"types":["axum::extract::form::Form"]},{"text":"impl&lt;B&gt; <a class=\"trait\" href=\"axum/extract/trait.FromRequest.html\" title=\"trait axum::extract::FromRequest\">FromRequest</a>&lt;B&gt; for <a class=\"struct\" href=\"axum/extract/struct.MatchedPath.html\" title=\"struct axum::extract::MatchedPath\">MatchedPath</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;B: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,&nbsp;</span>","synthetic":false,"types":["axum::extract::matched_path::MatchedPath"]},{"text":"impl&lt;T, B&gt; <a class=\"trait\" href=\"axum/extract/trait.FromRequest.html\" title=\"trait axum::extract::FromRequest\">FromRequest</a>&lt;B&gt; for <a class=\"struct\" href=\"axum/extract/struct.Query.html\" title=\"struct axum::extract::Query\">Query</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"serde/de/trait.DeserializeOwned.html\" title=\"trait serde::de::DeserializeOwned\">DeserializeOwned</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;B: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,&nbsp;</span>","synthetic":false,"types":["axum::extract::query::Query"]},{"text":"impl&lt;B&gt; <a class=\"trait\" href=\"axum/extract/trait.FromRequest.html\" title=\"trait axum::extract::FromRequest\">FromRequest</a>&lt;B&gt; for <a class=\"struct\" href=\"axum/extract/struct.RawQuery.html\" title=\"struct axum::extract::RawQuery\">RawQuery</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;B: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,&nbsp;</span>","synthetic":false,"types":["axum::extract::raw_query::RawQuery"]},{"text":"impl&lt;B&gt; <a class=\"trait\" href=\"axum/extract/trait.FromRequest.html\" title=\"trait axum::extract::FromRequest\">FromRequest</a>&lt;B&gt; for <a class=\"struct\" href=\"axum/extract/struct.OriginalUri.html\" title=\"struct axum::extract::OriginalUri\">OriginalUri</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;B: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,&nbsp;</span>","synthetic":false,"types":["axum::extract::request_parts::OriginalUri"]},{"text":"impl&lt;B&gt; <a class=\"trait\" href=\"axum/extract/trait.FromRequest.html\" title=\"trait axum::extract::FromRequest\">FromRequest</a>&lt;B&gt; for <a class=\"struct\" href=\"axum/extract/struct.BodyStream.html\" title=\"struct axum::extract::BodyStream\">BodyStream</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;B: <a class=\"trait\" href=\"http_body/trait.Body.html\" title=\"trait http_body::Body\">HttpBody</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + 'static,<br>&nbsp;&nbsp;&nbsp;&nbsp;B::<a class=\"associatedtype\" href=\"http_body/trait.Body.html#associatedtype.Data\" title=\"type http_body::Body::Data\">Data</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"struct\" href=\"bytes/bytes/struct.Bytes.html\" title=\"struct bytes::bytes::Bytes\">Bytes</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;B::<a class=\"associatedtype\" href=\"http_body/trait.Body.html#associatedtype.Error\" title=\"type http_body::Body::Error\">Error</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"type\" href=\"axum/type.BoxError.html\" title=\"type axum::BoxError\">BoxError</a>&gt;,&nbsp;</span>","synthetic":false,"types":["axum::extract::request_parts::BodyStream"]},{"text":"impl&lt;B&gt; <a class=\"trait\" href=\"axum/extract/trait.FromRequest.html\" title=\"trait axum::extract::FromRequest\">FromRequest</a>&lt;B&gt; for <a class=\"struct\" href=\"axum/extract/struct.RawBody.html\" title=\"struct axum::extract::RawBody\">RawBody</a>&lt;B&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;B: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,&nbsp;</span>","synthetic":false,"types":["axum::extract::request_parts::RawBody"]},{"text":"impl&lt;T, B&gt; <a class=\"trait\" href=\"axum/extract/trait.FromRequest.html\" title=\"trait axum::extract::FromRequest\">FromRequest</a>&lt;B&gt; for <a class=\"struct\" href=\"axum/extract/struct.TypedHeader.html\" title=\"struct axum::extract::TypedHeader\">TypedHeader</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"headers_core/trait.Header.html\" title=\"trait headers_core::Header\">Header</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;B: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,&nbsp;</span>","synthetic":false,"types":["axum::extract::typed_header::TypedHeader"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()
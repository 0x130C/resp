CREATE TYPE article_state AS ENUM ('draft', 'published');

CREATE TABLE article (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    slug VARCHAR NOT NULL,
    title VARCHAR NOT NULL,
    raw_content VARCHAR NOT NULL,
    content VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    state SMALLINT NOT NULL DEFAULT 0
);

CREATE TABLE category (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    "name" VARCHAR NOT NULL,
    slug VARCHAR NOT NULL
);

CREATE TABLE article_news_category_rel (
    article_id uuid REFERENCES article(id) ON UPDATE CASCADE ON DELETE CASCADE,
    cat_id uuid REFERENCES category(id) ON UPDATE CASCADE,
    PRIMARY KEY (article_id, cat_id)
);
INSERT INTO category(id, name, slug)
       VALUES ('fa3ecad0-58d6-4f94-ac3b-360543c1d196', 'Internet Technology', 'internet-technology'),
       ('fa3ecad0-58d6-4f94-ac3b-360543c1d200', 'Internet Technology', 'internet-technology'),
       ('fa3ecad0-58d6-4f94-ac3b-360543c1d201', 'Farmer', 'farmer'),
       ('fa3ecad0-58d6-4f94-ac3b-360543c1d202', 'Dark Hunter', 'dark-hunter');
INSERT INTO article(id, slug, title, raw_content, "content") VALUES
('fa3ecad0-58d6-4f94-ac3b-360543c1d100', 'uuid-or-guid-as-primary-keys-be-careful', 'UUID or GUID as Primary Keys? Be Careful!', '#\nUUID or GUID as Primary Keys? Be Careful!.', '<h1>UUID or GUID as Primary Keys? Be Careful!.</h1>'),
('fa3ecad0-58d6-4f94-ac3b-360543c1d101', 'uuid-or-guid-as-primary-keys-be-careful', 'UUID or GUID as Primary Keys? Be Careful!', '#\nUUID or GUID as Primary Keys? Be Careful!.', '<h1>UUID or GUID as Primary Keys? Be Careful!.</h1>'),
('fa3ecad0-58d6-4f94-ac3b-360543c1d102', 'uuid-or-guid-as-primary-keys-be-careful', 'UUID or GUID as Primary Keys? Be Careful!', '#\nUUID or GUID as Primary Keys? Be Careful!.', '<h1>UUID or GUID as Primary Keys? Be Careful!.</h1>'),
('fa3ecad0-58d6-4f94-ac3b-360543c1d103', 'uuid-or-guid-as-primary-keys-be-careful', 'UUID or GUID as Primary Keys? Be Careful!', '#\nUUID or GUID as Primary Keys? Be Careful!.', '<h1>UUID or GUID as Primary Keys? Be Careful!.</h1>'),
('fa3ecad0-58d6-4f94-ac3b-360543c1d104', 'uuid-or-guid-as-primary-keys-be-careful', 'UUID or GUID as Primary Keys? Be Careful!', '#\nUUID or GUID as Primary Keys? Be Careful!.', '<h1>UUID or GUID as Primary Keys? Be Careful!.</h1>'),
('fa3ecad0-58d6-4f94-ac3b-360543c1d105', 'uuid-or-guid-as-primary-keys-be-careful', 'UUID or GUID as Primary Keys? Be Careful!', '#\nUUID or GUID as Primary Keys? Be Careful!.', '<h1>UUID or GUID as Primary Keys? Be Careful!.</h1>');
INSERT INTO article_news_category_rel(article_id, cat_id)
       VALUES ('fa3ecad0-58d6-4f94-ac3b-360543c1d100', 'fa3ecad0-58d6-4f94-ac3b-360543c1d196');
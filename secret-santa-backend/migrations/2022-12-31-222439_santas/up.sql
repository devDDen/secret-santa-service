CREATE TABLE santas (
    id INT GENERATED ALWAYS AS IDENTITY,
    group_id INT NOT NULL,
    santa_id INT NOT NULL,
    recipient_id INT NOT NULL,
    CONSTRAINT unique_group_santa_recipient_id UNIQUE(group_id, santa_id, recipient_id),
    CONSTRAINT santas_pkey PRIMARY KEY(id),
    CONSTRAINT fk_santa FOREIGN KEY(santa_id) REFERENCES users(id) ON DELETE CASCADE,
    CONSTRAINT fk_recipient FOREIGN KEY(recipient_id) REFERENCES users(id) ON DELETE CASCADE,
    CONSTRAINT fk_group FOREIGN KEY(group_id) REFERENCES sgroups(id) ON DELETE CASCADE
);

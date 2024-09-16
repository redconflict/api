-- Enable UUID extension. 
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE SCHEMA RC;

-- USERS Contains all API users (including authentication data).
CREATE TABLE IF NOT EXISTS RC.USERS
(
  -- RowId: Record identifier (for joins).
  ID          BIGSERIAL,

  -- ID: Public ID for application usage.
  PUBLIC_ID   UUID        NOT NULL DEFAULT uuid_generate_v4(),

  USERNAME    VARCHAR(24) NOT NULL UNIQUE,
  PASSWD_HASH TEXT        NOT NULL,

  --CreatedAt: Filled by default when inserting new record.
  CREATED_AT  TIMESTAMPTZ NOT NULL DEFAULT NOW(),

  -- UpdatedAt: Filled when this record is updated.
  UPDATED_AT  TIMESTAMPTZ          DEFAULT NULL,

  -- DeletedAt: Soft delete, fill with timestamp when deleting this record.
  DELETED_AT  TIMESTAMPTZ          DEFAULT NULL,

  PRIMARY KEY (ID, PUBLIC_ID)
);

/*
DROP TABLE RC.USERS;

-- Insert new record in USERS table.
CREATE OR REPLACE FUNCTION RC.create_user(IN _username VARCHAR(24), IN _passwd_hash TEXT)
  RETURNS SETOF RC.USERS AS
$$
BEGIN
  RETURN QUERY INSERT INTO RC.USERS (USERNAME, PASSWD_HASH)
    VALUES (_username, _passwd_hash)
    RETURNING *;
  --RETURNING USERS.ID, USERS.USERNAME, USERS.PASSWD, USERS.created_at, USERS.updated_at, USERS.deleted_at;
END;
$$ LANGUAGE plpgsql;

SELECT * FROM RC.USERS;
*/

/*
-- Delete mark an existing user as
CREATE OR REPLACE FUNCTION RC.delete_user(_id VARCHAR(36))
  RETURNS TIMESTAMP AS
$$
DECLARE
  _tm TIMESTAMP;

BEGIN
  WITH r AS (UPDATE RC.USERS SET DELETED_AT = NOW() WHERE PUBLIC_ID = _id RETURNING DELETED_AT)
  SELECT r.DELETED_AT
    INTO _tm;

  RETURN _tm;
END;
$$ LANGUAGE plpgsql;

-- Remove function
-- DROP FUNCTION _Rc.create_user;

-- Test
SELECT *
  FROM RC.create_user(
    '9baffbe5-2b1c-422d-9b71-b35fb8b18755',
    'SpyMan',
    '$argon2i$v=19$m=16,t=2,p=1$ZHprdW9CUE1uMDVWc2g0bg$xVzV0vSX5vBia2466q5RZg');

DELETE
  FROM RC.USERS
  WHERE ROW_ID != 0;

*/

-- Add migration script here
-- Seed script for initial data

-- Insert a Super Admin user into tbl_admin
INSERT INTO tbl_admin (id, uname, pwd, email, admin_role, admin_stat, cid, mid)
VALUES
  (1, 'superoot', '#02#$argon2id$v=19$m=19456,t=2,p=1$tckO4K03TV+1Wt4454S6qA$9xuyg+TRLUixm+oyh+h4zBvJhFh/Yh+CBs0StzZTk6U', 'super@root.com', 'Super', 'Active', 1, 1);

-- Insert details for the Super Admin in tbl_details
INSERT INTO tbl_details (id, admin_id, first_name, last_name, birth_date, cid, mid)
VALUES
  (1, (SELECT id FROM tbl_admin WHERE uname = 'superoot'), 'Super', 'Admin', '1980-01-01', 1, 1);

-- Insert a record in tbl_permissions for the Super Admin with maximum permission level
INSERT INTO tbl_permissions (id, level, admin_id, role_id, cid, mid)
VALUES
  (1, 5, (SELECT id FROM tbl_admin WHERE uname = 'superoot'), 4, 1, 1);


INSERT INTO tbl_roles (role, level)
VALUES 
  ('VIEWER', 1)
  , ('MANAGES USERS', 2)
  , ('MANAGES ADMINS', 3)
  , ('MANAGES BOOKS', 4)
  , ('SUPER ADMIN', 5)

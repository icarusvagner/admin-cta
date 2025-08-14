-- Add migration script here

-- Tables
DROP TABLE IF EXISTS tbl_acc_removed;
DROP TABLE IF EXISTS tbl_acc_session;
DROP TABLE IF EXISTS tbl_categories;
DROP TABLE IF EXISTS tbl_category;
DROP TABLE IF EXISTS tbl_details;
DROP TABLE IF EXISTS tbl_permissions;
DROP TABLE IF EXISTS tbl_removed_acc_history;
DROP TABLE IF EXISTS tbl_admin;
DROP TABLE IF EXISTS tbl_book;
DROP TABLE IF EXISTS tbl_location;
DROP TABLE IF EXISTS tbl_itinerary_day;
DROP TABLE IF EXISTS tbl_itinerary_day_locations;
DROP TABLE IF EXISTS tbl_package;
DROP TABLE IF EXISTS tbl_package_itinerary;
DROP TABLE IF EXISTS tbl_package_pricing;
DROP TABLE IF EXISTS tbl_optional_activity;
DROP TABLE IF EXISTS tbl_itinerary_day_optional_activity;
DROP TABLE IF EXISTS tbl_inclusion;
DROP TABLE IF EXISTS tbl_package_inclusion;
DROP TABLE IF EXISTS tbl_guest;
DROP TABLE IF EXISTS tbl_booking;
DROP TABLE IF EXISTS tbl_review;

-- Types

DROP TYPE IF EXISTS session_typ;
DROP TYPE IF EXISTS admin_typ;
DROP TYPE IF EXISTS admin_stat;

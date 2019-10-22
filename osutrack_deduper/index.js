/**
 * Osu!track deduplication script.
 *
 * Connects to the database, iterates through the users.  Pulls down all updates for a user, finds update rows that
 * have no new data from the row before them, and builds up a list of IDs of duplicate rows.  Then, deletes all
 * of those duplicate rows.
 */
/* eslint-env node */

'use strict';

const mysql = require('mysql');
const _ = require('lodash');

const db_creds = require('./conf');
const dedupUser = require('./dedupUser');

const connection = mysql.createConnection({
  host: db_creds.db_host,
  user: db_creds.db_user,
  password: db_creds.db_pass,
  database: db_creds.db_database,
});
connection.connect(err => {
  if (err) {
    console.log('Error connecting to the database: ', err);
    process.exit(1);
  } else {
    console.log('Successfully connected to the database!');
  }
});

// Find the number of users in the database
const userOsuIdsQuery = 'SELECT `osu_id` from `users` WHERE 1 ORDER BY `osu_id` ASC;';
connection.query(userOsuIdsQuery, (err, res, fields) => {
  console.log(res[0]);
  // Loop through all users in the database
  res.map(_.property('osu_id')).forEach(osuId => dedupUser(osuId, connection));
});

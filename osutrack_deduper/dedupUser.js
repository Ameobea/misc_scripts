/**
 * Given a collection of updates for a user, returns the IDs of all rows that should be deleted.
 */

const _ = require('lodash');

const dedupUser = (id, connection) =>
  new Promise((f, r) => {
    // Pull down all update rows for the user form the database
    const userQuery = `SELECT * FROM \`updates\` WHERE \`user\` = ${id} ORDER BY \`id\` ASC;`;
    connection.query(userQuery, (err, res, _fields) => {
      if (err) {
        console.log('Error getting updates for user with id ' + id);
        process.exit(1);
      }

      // determine "duplicate" rows
      const duplicateIds = findDuplicates(res);

      // delete all duplicate rows.
      const dups = duplicateIds && duplicateIds.length !== 0 ? duplicateIds.join(', ') : null;
      if (!dups) {
        return f();
      }
      const deleteDuplicatesQuery = `DELETE FROM \`updates\` WHERE \`id\` IN (${dups});`;

      connection.query(deleteDuplicatesQuery, (err, res, _fields) => {
        if (err) {
          console.log(`Error while deleting duplicate rows from the database: ${err}`);
          r();
          process.exit(1);
        }

        console.log(`Deleted ${res.affectedRows} duplicate rows for user ${id}`);
        f();
      });
    });
  });

const findDuplicates = rows => {
  var lastRow = null;
  return _.reduce(
    rows,
    (acc, row) => {
      if (!lastRow) {
        lastRow = row;
        return [];
      }

      // Check to see if this row differs from the last row significantly
      if (
        row.mode === lastRow.mode &&
        row.playcount === lastRow.playcount &&
        row.pp_rank === lastRow.pp_rank &&
        row.total_score === lastRow.total_score
      ) {
        // Rows are essentially duplicate; mark the current row for deletion.
        return [...acc, row.id];
      } else {
        // This update has significance, so keep it.
        lastRow = row;
        return acc;
      }
    },
    []
  );
};

module.exports = dedupUser;

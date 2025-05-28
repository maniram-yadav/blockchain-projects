// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

contract TaxRecord {
    struct Record {
        uint256 id;
        string taxPayerName;
        uint256 amount;
        uint256 date;
        bool exists;
    }

    mapping(uint256 => Record) private records;
    uint256 public nextId = 1;
    address public owner;

    event RecordAdded(uint256 indexed id, string taxPayerName, uint256 amount, uint256 date);
    event RecordDeleted(uint256 indexed id);

    function addRecord(string memory _taxPayerName, uint256 _amount) public returns (uint256) {
        uint256 id = nextId++;
        records[id] = Record(id, _taxPayerName, _amount, block.timestamp, true);
        emit RecordAdded(id, _taxPayerName, _amount, block.timestamp);
        return id;
    }

    function getRecord(uint256 _id) public view returns (Record memory) {
        require(records[_id].exists, "Record does not exist");
        return records[_id];
    }

    function getAllRecords() public view returns (Record[] memory) {
        Record[] memory allRecords = new Record[](nextId - 1);
        uint256 counter = 0;
        for (uint256 i = 1; i < nextId; i++) {
            if (records[i].exists) {
                allRecords[counter] = records[i];
                counter++;
            }
        }

        // Resize the array to exclude empty slots
        Record[] memory resizedRecords = new Record[](counter);
        for (uint256 i = 0; i < counter; i++) {
            resizedRecords[i] = allRecords[i];
        }

        return resizedRecords;
    }

    function deleteRecord(uint256 _id) public {
        require(records[_id].exists, "Record does not exist");
        delete records[_id];
        emit RecordDeleted(_id);
    }

    // Getter to return the next record ID
    function nextRecordId() public view returns (uint256) {
        return nextId;
    }
}

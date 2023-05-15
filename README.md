Laboratory Information Management System (LIMS) implemented in Rust. It supports various features for managing samples, analyses, users, inventory items, scheduling tests, and recording quality control. Frontend coming soon.

Features:

Basic CRUD for samples:

- Adding a sample with the add_sample function.
- Adding an analysis for a sample with the add_analysis function.
- Updating the description of a sample with the update_sample_description function.
- Deleting a sample with the delete_sample function.
- Querying and reporting samples using the get_samples function.

Batch Operations:
- Adding multiple samples in a single transaction with the add_samples function.

User Management:
- Adding a user with the add_user function.
- Updating the role of a user with the update_user_role function.
- Deleting a user with the delete_user function.

Inventory Management:
- Adding an inventory item with the add_inventory_item function.
- Updating the quantity of an inventory item with the update_inventory_item_quantity function.
- Deleting an inventory item with the delete_inventory_item function.

Scheduling and Workflow Management:
- Adding a test with the add_test function.
- Scheduling a test for a sample with the schedule_test function.

Quality Control and Assurance:
- Recording quality control for a sample and test with the record_quality_control function.

The application provides a main function that initializes the database, adds a sample and an analysis statically, and demonstrates the usage of the implemented functions.

Overall, this LIMS application provides a foundation for managing laboratory-related data, such as samples, analyses, users, inventory, test scheduling, and quality control.

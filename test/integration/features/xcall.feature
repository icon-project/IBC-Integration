Feature:  Xcall-Admin
    In order to set fee
    Owner should add admin
    Who will have access to set fee

    Scenario: Owner Adds Admin
        Given Admin Address to be added
        """
        {"set_admin":{"address":"archway12758s43wawjy4kj5p7wzmq6tw8syxndvah7xd2"}}
        """
        When Owner adds admin
        # This can be tested by querying get_admin
        Then Admin should be added successfully
        """
        get_admin
        """
    
    # Scenario: Non Owner adds admin
    #     Given Admin Address to be added
    #     """
    #     {"set_admin":{"address":"archway12758s43wawjy4kj5p7wzmq6tw8syxndvah7xd2"}}
    #     """
    #     When Non Owner adds admin
    #     Then Admin should not be added successfully
        
    
    
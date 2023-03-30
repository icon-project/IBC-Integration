/*
 * Copyright (c) 2022-2022 Balanced.network.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package ibc.icon.test;

import com.iconloop.score.test.Account;
import com.iconloop.score.test.Score;
import com.iconloop.score.test.ServiceManager;
import org.mockito.Mockito;
import score.Address;

public class MockContract<T> {
    final private static Address SCORE_ZERO = Address.fromString("cx" + "0".repeat(40));
    public final Account account;
    public final T mock;

    public MockContract(Class<? extends T> classToMock, ServiceManager sm, Account admin) throws Exception {
        mock = Mockito.mock(classToMock);
        Score score = sm.deploy(admin, classToMock, SCORE_ZERO);
        score.setInstance(mock);
        account = score.getAccount();
    }

    public MockContract(Class<? extends T> classToMock, Class<T> mockClass, ServiceManager sm, Account admin) throws Exception {
        mock = Mockito.mock(mockClass);
        Score score = sm.deploy(admin, classToMock, SCORE_ZERO);
        score.setInstance(mock);
        account = score.getAccount();
    }

    public Address getAddress() {
        return account.getAddress();
    }
}
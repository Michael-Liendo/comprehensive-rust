// Copyright 2022 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// ANCHOR: IBirthdayService
//! Implementation of the `IBirthdayService` AIDL interface.
use binder::{ParcelFileDescriptor, SpIBinder, Strong};
use com_example_birthdayservice::aidl::com::example::birthdayservice::IBirthdayInfoProvider::{IBirthdayInfoProvider, BpBirthdayInfoProvider};
use com_example_birthdayservice::aidl::com::example::birthdayservice::IBirthdayService::{IBirthdayService};
use com_example_birthdayservice::aidl::com::example::birthdayservice::BirthdayInfo::BirthdayInfo;
use com_example_birthdayservice::binder;
use std::fs::File;
use std::io::Read;

/// The `IBirthdayService` implementation.
pub struct BirthdayService;

impl binder::Interface for BirthdayService {}

impl IBirthdayService for BirthdayService {
    fn wishHappyBirthday(&self, name: &str, years: i32) -> binder::Result<String> {
        Ok(format!("Happy Birthday {name}, congratulations with the {years} years!"))
    }

    fn wishWithInfo(&self, info: &BirthdayInfo) -> binder::Result<String> {
        Ok(format!(
            "Happy Birthday {}, congratulations with the {} years!",
            info.name, info.years,
        ))
    }

    fn wishWithProvider(
        &self,
        provider: &Strong<dyn IBirthdayInfoProvider>,
    ) -> binder::Result<String> {
        Ok(format!(
            "Happy Birthday {}, congratulations with the {} years!",
            provider.name()?,
            provider.years()?,
        ))
    }

    fn wishWithErasedProvider(&self, provider: &SpIBinder) -> binder::Result<String> {
        use binder::binder_impl::Proxy;

        // Convert the `IBinder` to a concrete interface.
        let provider = BpBirthdayInfoProvider::from_binder(provider.clone())?;

        Ok(format!(
            "Happy Birthday {}, congratulations with the {} years!",
            provider.name()?,
            provider.years()?,
        ))
    }

    fn wishFromFile(&self, info_file: &ParcelFileDescriptor) -> binder::Result<String> {
        let mut info_file = info_file
            .as_ref()
            .try_clone()
            .map(File::from)
            .expect("Invalid file handle");

        let mut contents = String::new();
        info_file.read_to_string(&mut contents).unwrap();

        let mut lines = contents.lines();
        let name = lines.next().unwrap();
        let years: i32 = lines.next().unwrap().parse().unwrap();

        Ok(format!(
            "Happy Birthday {name}, congratulations with the {years} years!"
        ))
    }
}

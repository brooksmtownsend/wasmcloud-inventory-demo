import React, { useState, useEffect } from 'react';
import {
  ChakraProvider,
  Box,
  Text,
  VStack,
  Grid,
  theme,
  Button,
  Menu, MenuButton, MenuList, MenuItem,
  Table, Thead, Tbody, Tr, Th, Td, HStack
} from '@chakra-ui/react';
import { ColorModeSwitcher } from './ColorModeSwitcher';
import { Logo } from './Logo';

const WASMCLOUD_COLOR = "#00C389";
const backupData = [
  { id: 1, branch: "N/A", item_type: "None", quantity: 0 },
];

function App() {
  const [inventoryData, setInventoryData] = useState([]);

  const fetchInventory = () => {
    // Fetch data from your API endpoint
    fetch("/inventory")
      .then((response) => response.json())
      .then((data) => setInventoryData(data.flat(Infinity).sort((a, b) => a.branch.localeCompare(b.branch))))
      .catch((error) => {
        setInventoryData(backupData);
        console.error("Error fetching data:", error);
      });
  };

  const fetchRundown = () => {
    fetch("/rundown")
      .then((response) => console.dir(response))
      .catch((error) => {
        setInventoryData(backupData);
        console.error("Error fetching data:", error);
      });
  };

  const continualFetch = () => {
    fetchRundown();
    setTimeout(fetchInventory, 1000);
  };

  const [selectedItem, setSelectedItem] = React.useState(null);

  const handleMenuItemClick = (itemValue) => {
    setSelectedItem(itemValue);
  };


  useEffect(() => {
    fetchInventory(); // Fetch data when the component mounts
    setInterval(continualFetch, 3000); // Continually poll for updates
  }, []);

  return (
    <ChakraProvider theme={theme}>
      <Box textAlign="center" fontSize="xl">
        <Grid p={3}>
          <ColorModeSwitcher justifySelf="flex-end" />
          <VStack spacing={4}>
            <Logo h="200px" pointerEvents="none" />
            <Box width="100%">
              <Box overflowX="auto" textAlign="center" mb={4}>
                <HStack justifyContent="center">
                  <Text>
                    Hub Dashboard
                  </Text>
                  <Menu>
                    {({ isOpen }) => (
                      <>
                        <MenuButton isActive={isOpen} as={Button} rightIcon="▼">
                          Select Branch
                        </MenuButton>
                        <MenuList>
                          <MenuItem onClick={() => handleMenuItemClick(null)}>All</MenuItem>
                          {[...new Set(inventoryData.map((i) => i.branch))].map((item) => (
                            <MenuItem key={item} onClick={() => handleMenuItemClick(item)}>{item}</MenuItem>
                          ))}
                        </MenuList>
                      </>
                    )}
                  </Menu>
                </HStack>
                <Table variant="simple" width="100%" maxWidth="50vw" mx="auto">
                  <Thead>
                    <Tr>
                      <Th>Branch</Th>
                      <Th>Item</Th>
                      <Th textAlign="right">Quantity</Th> {/* Right-align Quantity column */}
                    </Tr>
                  </Thead>
                  <Tbody>
                    {inventoryData.filter((item) => {
                      if (!selectedItem) {
                        return true;
                      } else {
                        return item.branch === selectedItem;
                      }
                    }).map((item) => (
                      <Tr key={item.id}>
                        <Td>{item.branch}</Td>
                        <Td>{item.item_type}</Td>
                        <Td textAlign="right">{item.quantity}</Td> {/* Right-align Quantity cell */}
                      </Tr>
                    ))}
                  </Tbody>
                </Table>
              </Box>
            </Box>
          </VStack>
        </Grid>
      </Box>
    </ChakraProvider >
  );
}

export default App;
